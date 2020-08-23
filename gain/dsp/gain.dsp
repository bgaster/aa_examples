declare name        "volume";
declare version     "1.0";
declare author      "Grame";
declare license     "BSD";
declare copyright   "(c)GRAME 2006";

//-----------------------------------------------
//          Volume control in dB
//-----------------------------------------------

import("stdfaust.lib");

gain        = vslider("gain_control", 0, -70, +4, 0.1) : ba.db2linear : si.smoo;

process     = *(gain);