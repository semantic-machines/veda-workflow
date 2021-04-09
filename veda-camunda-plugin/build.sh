BUILD_PATH=$PWD
mkdir ./tmp
cd tmp
git clone https://github.com/semantic-machines/v-queue
cd v-queue/java
./build.sh
cd $BUILD_PATH
mvn clean install
