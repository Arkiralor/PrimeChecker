## Usage: sh scrpts/build.sh --arg=release

arg="$1"

if [[ "$arg" == "release" ]]; then
    echo "Building release version..."
    cargo build --release
    echo "...build complete."
elif [[ "$arg" == "debug" ]]; then
    echo "Building debug version..."
    cargo build
    echo "...build complete."
elif [[ "$arg" == "both" ]]; then
    echo "Building BOTH versions..."
    echo "Building release version..."
    cargo build --release
    echo "...build finished."
    echo "Building debug version..."
    cargo build
    echo "...build finished."
    echo "...build complete."
else
    if [[ "$arg" == "" ]]; then
        arg="NULL"
    elif [ -z "$arg" ]; then
        arg="NULL"
    fi
    echo "Unknown build type:    $arg."
fi