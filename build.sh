BUILD_PATH=$PWD

mkdir ./bin
VEDA_BIN=$BUILD_PATH/bin

if [ $1 == "camunda-user-task" ] || [ $1 == "veda-camunda-user-task" ] || [ -z $1 ]; then
    echo BUILD veda-camunda-user-task
    rm ./veda-camunda-user-task

    cd veda-camunda-user-task
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/veda-camunda-user-task $VEDA_BIN

fi

if [ $1 == "camunda-external-task" ] || [ $1 == "veda-camunda-external-task" ] || [ -z $1 ]; then
    echo BUILD veda-camunda-external-task
    rm ./veda-camunda-external-task

    cd veda-camunda-external-task
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/veda-camunda-external-task $VEDA_BIN

fi

if [ $1 == "camunda-connector" ] || [ $1 == "veda-camunda-connector" ] || [ -z $1 ]; then
    echo BUILD veda-camunda-connector
    rm ./veda-camunda-connector

    cd veda-camunda-connector
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/veda-camunda-connector $VEDA_BIN

fi

