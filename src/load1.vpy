import vapoursynth as vs
core = vs.core
clip = core.lsmas.LWLibavSource(source="C:\Users\Quack\Videos\test\lum-test\video.mkv")
#clip = core.resize.Bicubic(clip=clip, format=vs.RGB48)
clip.set_output()
