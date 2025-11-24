#include "eph.h"
#include "const.h"
#include <math.h>
#include <string>
//=================================数学工具=========================================

// 赤道坐标转为地平坐标;
Vector3 CD2DP(Vector3 z, long double L, long double fa, long double gst)
{
	// 转到相对于地平赤道分点的赤道坐标;
	Vector3 a(z.x + PI / 2 - gst - L, z.y, z.z);
	a = llrConv(a, PI / 2 - fa);
	a.x = rad2mrad(PI / 2 - a.x);
	return a;
}

// 求角度差;
long double j1_j2(long double J1, long double W1, long double J2, long double W2)
{
	long double dJ = rad2rrad(J1 - J2), dW = W1 - W2;
	if (fabs(dJ) < 1 / 1000 && fabs(dW) < 1 / 1000)
	{
		dJ *= cos((W1 + W2) / 2);
		return sqrt(dJ * dJ + dW * dW);
	}
	return acos(sin(W1) * sin(W2) + cos(W1) * cos(W2) * cos(dJ));
}

// 视差角(不是视差);
long double shiChaJ(long double gst, long double L, long double fa, long double J, long double W)
{
	long double H = gst + L - J; // 天体的时角;
	return rad2mrad(atan2(sin(H), tan(fa) * cos(W) - sin(W) * cos(H)));
}

//=================================蒙气改正=========================================
//==================================================================================
long double MQC(long double h) { return 0.0002967 / tan(h + 0.003138 / (h + 0.08919)); }	  // 大气折射,h是真高度
long double MQC2(long double ho) { return -0.0002909 / tan(ho + 0.002227 / (ho + 0.07679)); } // 大气折射,ho是视高度

//=================================视差改正=========================================
//==================================================================================
void parallax(Vector3 z, long double H, long double fa, long double high)
{ // 视差修正
	// z赤道坐标,fa地理纬度,H时角,high海拔(千米)
	long double dw = 1;
	if (z[2] < 500)
		dw = cs_AU;
	z[2] *= dw;
	long double r0, x0, y0, z0, f = cs_ba, u = atan(f * tan(fa)), g = z[0] + H;
	r0 = cs_rEar * cos(u) + high * cos(fa);		// 站点与地地心向径的赤道投影长度
	z0 = cs_rEar * sin(u) * f + high * sin(fa); // 站点与地地心向径的轴向投影长度
	x0 = r0 * cos(g);
	y0 = r0 * sin(g);

	Vector3 s = llr2xyz(z);
	s[0] -= x0, s[1] -= y0, s[2] -= z0;
	s = xyz2llr(s);
	z[0] = s[0], z[1] = s[1], z[2] = s[2] / dw;
}

	// 转入地平纬度及地月质心距离,返回站心视半径(角秒)
	long double moonRad(long double r, long double h)
	{
		return cs_sMoon / r * (1 + sin(h) * cs_rEar / r);
	}

	// 求月亮近点时间和距离,t为儒略世纪数力学时
	Vector2 moonMinR(long double t, long double min)
	{
		long double a = 27.55454988 / 36525, b;
		if (min)
			b = -10.3302 / 36525;
		else
			b = 3.4471 / 36525;
		t = b + a * int2((t - b) / a + 0.5); // 平近(远)点时间
		long double r1, r2, r3, dt;
		// 初算二次
		dt = 1 / 36525;
		r1 = XL1_calc(2, t - dt, 10);
		r2 = XL1_calc(2, t, 10);
		r3 = XL1_calc(2, t + dt, 10);
		t += (r1 - r3) / (r1 + r3 - 2 * r2) * dt / 2;
		dt = 0.5 / 36525;
		r1 = XL1_calc(2, t - dt, 20);
		r2 = XL1_calc(2, t, 20);
		r3 = XL1_calc(2, t + dt, 20);
		t += (r1 - r3) / (r1 + r3 - 2 * r2) * dt / 2;
		// 精算
		dt = 1200 / 86400 / 36525;
		r1 = XL1_calc(2, t - dt, -1);
		r2 = XL1_calc(2, t, -1);
		r3 = XL1_calc(2, t + dt, -1);
		t += (r1 - r3) / (r1 + r3 - 2 * r2) * dt / 2;
		r2 += (r1 - r3) / (r1 + r3 - 2 * r2) * (r3 - r1) / 8;
		Vector2 re(t, r2);
		return re;
	}

	Vector3 moonNode(long double t, long double asc)
	{ // 月亮升交点
		long double a = 27.21222082 / 36525, b;
		if (asc)
			b = 21 / 36525;
		else
			b = 35 / 36525;
		t = b + a * int2((t - b) / a + 0.5); // 平升(降)交点时间
		long double w, v, w2, dt;
		dt = 0.5 / 36525;
		w = XL1_calc(1, t, 10);
		w2 = XL1_calc(1, t + dt, 10);
		v = (w2 - w) / dt;
		t -= w / v;
		dt = 0.05 / 36525;
		w = XL1_calc(1, t, 40);
		w2 = XL1_calc(1, t + dt, 40);
		v = (w2 - w) / dt;
		t -= w / v;
		w = XL1_calc(1, t, -1);
		t -= w / v;
		Vector3 re; // (t, XL1_calc(0, t, -1));
		return re;
	}

	// 地球近远点
	Vector2 earthMinR(long double t, long double min)
	{
		long double a = 365.25963586 / 36525, b;
		if (min)
			b = 1.7 / 36525;
		else
			b = 184.5 / 36525;
		t = b + a * int2((t - b) / a + 0.5); // 平近(远)点时间
		long double r1, r2, r3, dt;
		// 初算二次
		dt = 3 / 36525;
		r1 = XL0_calc(0, 2, t - dt, 10);
		r2 = XL0_calc(0, 2, t, 10);
		r3 = XL0_calc(0, 2, t + dt, 10);
		t += (r1 - r3) / (r1 + r3 - 2 * r2) * dt / 2; // 误差几个小时
		dt = 0.2 / 36525;
		r1 = XL0_calc(0, 2, t - dt, 80);
		r2 = XL0_calc(0, 2, t, 80);
		r3 = XL0_calc(0, 2, t + dt, 80);
		t += (r1 - r3) / (r1 + r3 - 2 * r2) * dt / 2; // 误差几分钟
		// 精算
		dt = 0.01 / 36525;
		r1 = XL0_calc(0, 2, t - dt, -1);
		r2 = XL0_calc(0, 2, t, -1);
		r3 = XL0_calc(0, 2, t + dt, -1);
		t += (r1 - r3) / (r1 + r3 - 2 * r2) * dt / 2; // 误差小于秒
		r2 += (r1 - r3) / (r1 + r3 - 2 * r2) * (r3 - r1) / 8;
		Vector2 re(t, r2);
		return re;
	}
};
