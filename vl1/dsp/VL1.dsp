declare aavoices "1";

import("stdfaust.lib");

waveforms = piano, fantasy, violin, flute,
  guitar1 : ba.selectn(5, wave)
with {
  wave = nentry("/Waveform", 0, 0, 4, 1);
  freq = hslider("freq", 440, 50, 2000, 0.01);
  gain = hslider("gain", 0.5, 0, 1, 0.01);
  g = button("gate");

  piano = os.pulsetrain(freq, 0.7) * envelope : filter
  with{
	envelope = en.adsr(0.005, 0.178, 0.306, 0.178, g) * gain;
	filter = fi.resonlp(3000,0.5,gain);
  };

  fantasy = os.pulsetrain(2*freq + vibrato, 0.5) * envelope : filter
  with {
	envelope = en.adsr(0.005, 0.05, 0.830, 1.303, g) * gain;
	filter = fi.resonlp(3000,0.5,gain);
	vibrato = os.lf_triangle(5.7)*10;
  };

  violin = ((os.pulsetrain(freq + vibrato, 0.5) * envelope) + (os.pulsetrain(freq + vibrato, 0.4) * envelope) + (os.pulsetrain(freq + vibrato, 0.3) * envelope) + (os.pulsetrain(freq + vibrato, 0.2) * envelope) + (os.pulsetrain(freq + vibrato, 0.1) * envelope))/5 : filter
  with {
	envelope = en.adsr(0.05, 0.05, 0.830, 0.25, g) * gain;
	filter = fi.resonlp(3500,5,gain);
	vibrato = os.lf_triangle(5.7)*5;
  };

  flute = os.pulsetrain(freq + vibrato, 0.5) * envelope : filter
  with {
	envelope = en.adsr(0.05, 0.05, 0.830, 0.25, g) * gain;
	filter = fi.resonlp(3000,0.5,gain);
	vibrato = os.lf_triangle(5.7)*3;
  };

  guitar1 = (os.pulsetrain(0.5*freq, 0.1) * envelope + os.pulsetrain(0.5*freq, 0.2) * envelope)/2  : filter
  with {
	envelope = en.adsr(0.03, 0.05, 0.830, 0.25, g) * gain;
	filter = fi.resonlp(3500,2,gain);
  };
};

process = vgroup("voices", par(n, 1, vgroup("aavoice%n", waveforms))) :> _ ;

//process = waveforms : _ ;
