build_server_module () {
    BUILD_PATH=$PWD
    VEDA_BIN=$BUILD_PATH/bin
    module_name=$1

    echo $module_name
    rm ./$module_name

    cd $module_name
    cargo build --release
    status=$?
    if test $status -ne 0
    then
        exit $status;
    fi
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/$module_name $VEDA_BIN
}

BUILD_PATH=$PWD

mkdir ./bin
VEDA_BIN=$BUILD_PATH/bin

if [ $1 == "camunda-event-handler" ] || [ $1 == "veda-camunda-event-handler" ] || [ -z $1 ]; then
    build_server_module "veda-camunda-event-handler"
fi

if [ $1 == "camunda-external-task-worker" ] || [ $1 == "veda-camunda-external-task-worker" ] || [ -z $1 ]; then
    build_server_module "veda-camunda-external-task-worker"
fi

if [ $1 == "camunda-process-controller" ] || [ $1 == "veda-camunda-process-controller" ] || [ -z $1 ]; then
    build_server_module "veda-camunda-process-controller"
fi

