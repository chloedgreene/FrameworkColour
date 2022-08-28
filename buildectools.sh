cd framework-ec

echo Building ectool

# As the Deprication of net-tools we trick the "hostname" command to run
echo Dummy Created
alias hostname="echo dummy"

echo mk utils -j2
make utils -j2

echo coping results

cp ./build/bds/util/ectool ../tools/ectool