# lsk-shorty
Pure Rust-tool to brute-force short Lisk addresses.

Requires Rust 1.25.0: https://rustup.rs/

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

Change `NTHREADS` to the number of your processor cores and make sure you compile it in optimized `--release` mode:

```bash
$ cargo build --release -j $(nproc)
$ ./target/release/lsk-shorty
```

Example output:

```bash
#7	20	5853242781580775517L	jacket force airport amateur brush negative below climb among jewel lounge theory	348	348.0/s/t
#7	19	355806560430640084L	february ugly lion trip exile tennis antique hold together demise lottery height	480	480.0/s/t
#6	19	902392753207102483L	hidden metal winter across angle patch tissue there quantum scatter sorry subject	788	394.0/s/t
#1	18	73613400984067281L	witness hour unique rigid address spy deposit critic cup increase office fox	1752	438.0/s/t
#6	18	29881964673140242L	stamp erosion license broccoli another invite grain owner still poet bronze wool	1874	468.5/s/t
#4	18	31613074924490389L	hotel world arch excite leave boost vibrant wedding trade garlic ozone notable	1890	472.5/s/t
#3	18	96465682394554229L	audit utility wall donate digital together someone vague damp series market faculty	1993	398.6/s/t
#5	17	2752497123342746L	total traffic monkey tomato maximum supply pride empty palm report amused display	2020	404.0/s/t
#7	18	85067416372355851L	trigger alley throw chronic hip rotate layer afford cable elder hour sniff	2483	413.8/s/t
#3	17	9225836226174129L	throw crush believe purse proud today enhance quarter change prize horse garden	2825	403.6/s/t
#7	17	7042230114326317L	boat suffer stable clerk romance harsh woman aspect web milk spirit liberty	2905	415.0/s/t
#6	17	9883020663018061L	hurdle provide enlist flight fox surround combine lumber toy august ethics course	3723	465.4/s/t
#2	16	136259720644981L	paper napkin pride ensure dutch aunt giggle right donor outer warm now		4112	456.9/s/t
#1	17	4918100684320144L	nature install word manage relief kitchen assume topic trash pear index actress	19817	460.9/s/t
#0	15	70543604248679L		prevent fantasy infant surround ramp own envelope ethics drama bind envelope left	23025	434.4/s/t
#1	16	194915900853054L	differ vital labor length jewel dutch together differ grit enforce balcony palm	31686	434.1/s/t
#4	17	6263914286079989L	tenant divide catch clarify rug letter oval copper horn grace grocery surface	32537	433.8/s/t
#7	16	624118465279291L	unusual lava deal rare cannon insane struggle film burst seat taxi spy		42483	442.5/s/t
#4	16	140830996527648L	drink describe protect clog mosquito under disorder rack cannon energy crush hospital	88450	460.7/s/t
#5	16	452590077103614L	absorb hawk tonight deliver dawn hood shadow castle input come calm raccoon	149129	469.0/s/t
#3	16	769892043282202L	diesel jazz mandate focus shaft sight wool funny ready grain syrup right	172038	471.3/s/t
#5	14	2623831930347L		outdoor expand busy auto talk toward leg actual illegal air offer alcohol	199549	471.7/s/t
#6	16	751036083841279L	riot travel arch distance taxi quality loop muffin shiver solution butter elegant	201401	471.7/s/t
#6	15	20397181825258L		useless split oak chunk average student eager final exotic rather solve know	320032	476.9/s/t
#7	15	50684771938048L		wrist flush odor bench practice stadium tennis rely always vault repair asthma	401915	478.5/s/t
#4	15	37609769642761L		bamboo soccer spot save blade cereal giraffe thank want glare oblige buddy	559720	480.4/s/t
#1	15	66168495934123L		walnut tell purpose garden core alpha right same penalty define trick where	589589	480.9/s/t
#3	15	57517022377296L		dish category rail mutual odor gun rude believe dismiss estate betray satisfy	1799713	451.2/s/t
#7	14	9431523159340L		skull still cannon door noodle rotate tired remind dutch game quote party	2208974	451.3/s/t
#6	14	6203081381290L		weather honey embrace cross notable sunny sign fold thank know heavy need	4009431	442.8/s/t
#2	13	278119600997L		maze leopard jump story gain minimum motion comic wish life wasp sad		4727109	440.7/s/t
#4	14	5511958756431L		cereal account reject ice velvet shiver announce junior help month kick witness	6420852	447.0/s/t
#1	14	7513837937679L		current improve employ ride park divert average sword scrap derive defy liquid	18204640	489.2/s/t
```

Happy brute-forcing!
