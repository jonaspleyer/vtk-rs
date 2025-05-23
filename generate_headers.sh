cxxbridge >libvtkrs/cxx.h

for file in src/vt*.rs; do
    cxxbridge $file --header >$file.h
    mv $file.h libvtkrs
done
