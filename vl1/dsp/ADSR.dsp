declare aavoices "1";

import("stdfaust.lib");

gain = hslider("gain", 0.5, 0, 1, 0.01): si.smoo;

waveforms = piano, fantasy, violin, flute,
  guitar1, guitar2, horn, electro1,
  electro2, electro3 : ba.selectn(10, wave)
with {
  wave = nentry("/Waveform", 0, 0, 9, 1);
  freq = hslider("freq", 440, 50, 2000, 0.01);

  piano = os.pulsetrain(freq + vibrato, 0.7) * envelope * tremelo : filter
  with{
	filter = fi.resonlp(3000,0.5,gain);
  };

  fantasy = os.pulsetrain(2*freq + vibrato, 0.5) * envelope * tremelo : filter
  with {
	filter = fi.resonlp(3000,0.5,gain);
  };

  violin = ((os.pulsetrain(freq + vibrato, 0.5) * envelope * tremelo) + (os.pulsetrain(freq + vibrato, 0.4) * envelope * tremelo) + (os.pulsetrain(freq + vibrato, 0.3) * envelope * tremelo) + (os.pulsetrain(freq + vibrato, 0.2) * envelope * tremelo) + (os.pulsetrain(freq + vibrato, 0.1) * envelope * tremelo))/5 : filter
  with {
	filter = fi.resonlp(3500,5,gain);
  };

  flute = os.pulsetrain(freq + vibrato, 0.5) * envelope * tremelo : filter
  with {
	filter = fi.resonlp(3000,0.5,gain);
  };

  guitar1 = (os.pulsetrain(0.5*freq + vibrato, 0.1) * envelope * tremelo + os.pulsetrain(0.5*freq + vibrato, 0.2) * envelope * tremelo)/2  : filter
  with {
	filter = fi.resonlp(3500,1,gain);
  };

  guitar2 = (os.pulsetrain(0.5*freq + vibrato, 0.5) * envelope * tremelo + os.pulsetrain(0.5*freq + vibrato, 0.25) * envelope * tremelo + os.pulsetrain(0.5*freq + vibrato, 0.125) * envelope * tremelo) /3 : filter
  with {
	filter = fi.resonlp(3000,2,gain);
  };

  horn = os.pulsetrain(0.5*freq + vibrato, 0.125) * envelope * tremelo : filter
  with{
	filter = fi.resonlp(3000,0.5,gain);
  };

  electro1 = os.pulsetrain(freq + vibrato + (pitchMod*220), 0.7) * envelope * tremelo: filter
  with{
	filter = fi.resonlp(3000,0.5,gain);
  };

  electro2 = os.pulsetrain(2*freq + vibrato + (pitchMod*440), 0.5) * envelope * tremelo : filter
  with{
	filter = fi.resonlp(3000,0.5,gain);
  };

  electro3 = ((os.pulsetrain(freq + vibrato + (pitchMod*220), 0.5)* envelope * tremelo) + (os.pulsetrain(freq + vibrato + (pitchMod*220), 0.4) * envelope * tremelo) + (os.pulsetrain(freq + vibrato + (pitchMod*220), 0.3) * envelope * tremelo) + (os.pulsetrain(freq + vibrato + (pitchMod*220), 0.2) * envelope * tremelo) + (os.pulsetrain(freq + vibrato + (pitchMod*220), 0.1) * envelope * tremelo))/5 : filter
  with {
	filter = fi.resonlp(3500,10,gain);
  };
};

vibrato = os.lf_triangle(5.7)*vdepth
with {
  vdepth = nentry("/Vibrato", 0, 0, 10, 1);
};

tremelo = os.lf_triangle(tremFreq)*5
with {
  tremFreq = nentry("/Tremelo", 0, 0, 10, 1);
};

pitchMod = os.lf_squarewavepos(modFreq)*-1
with {
  modFreq = nentry("/PitchMod", 4.5,2.7, 10, 0.365);
};

envelope = en.adsr(a,d,s,r,g) * gain
  with {
    a = nentry("/Attack", 0.005, 0.001, 0.5,0.05);
    d = nentry("/Decay", 0.178, 0.001, 2, 0.2);
    s = nentry("/Sustain", 0.306, 0.1, 1, 0.1);
    r = nentry("/Release", 0.178, 0.001, 2, 0.2);
    g = button("gate") : si.smoo;
  };

process = vgroup("voices", par(n, 1, vgroup("aavoice%n", waveforms))) : _;
