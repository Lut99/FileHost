#!/bin/bash
# SETUP.sh
#   by Lut99
#
# Created:
#   16 Apr 2022, 14:48:08
# Last edited:
#   06 Jun 2022, 12:05:33
# Auto updated?
#   Yes
#
# Description:
#   Script that manages installation and removal of the server-side of the
#   FileHost.
#


##### CONSTANTS #####
# The version of this script
SCRIPT_VERSION="0.1.0"

# The username of the special filehost user
USER="filehost"
# The URL to download files from
REPOSITORY="https://github.com/Lut99/FileHost/releases/download"
# The systemd entry name
SERVICE_ENTRY_NAME="filehostd.service"
# The systemd entry location
SERVICE_ENTRY="/etc/systemd/system/$SERVICE_ENTRY_NAME"
# The systemd socket entry name
SOCKET_ENTRY_NAME="filehostd.socket"
# The systemd socket entry location
SOCKET_ENTRY="/etc/systemd/system/$SOCKET_ENTRY_NAME"

# The default CTL location
CTL_BIN="/usr/bin/filehostctl"
# The default server location
SERVER_BIN="/usr/sbin/filehostd"
# The default socket path
SOCKET="/run/filehost/ctl.sock"
# The default config location
CONFIG="/etc/filehost/config.json"





##### ARGUMENT PARSING #####
# Define default values, if any
local_ctl=""
local_server=""
ctl_bin="$CTL_BIN"
server_bin="$SERVER_BIN"
socket="$SOCKET"
config="$CONFIG"
mode="install"
version="latest"

# Iterate over the arguments
accept_options=1
error=0
pos_i=0
state="start"
for arg in "$@"; do
    if [[ "$state" == "start" ]]; then
        # Switch on whether it's an option or not
        if [[ accept_options -eq 1 && "$arg" =~ ^- ]]; then
            # It's an option
            
            # Switch on its value
            if [[ "$arg" == "-l" || "$arg" == "--local-ctl" ]]; then
                # It's given, so use the next one instead
                state="local_ctl"

            elif [[ "$arg" == "-L" || "$arg" == "--local-server" ]]; then
                # It's given, so use the next one instead
                state="local_server"

            elif [[ "$arg" == "-C" || "$arg" == "--ctl-bin" ]]; then
                # Get the next argument as the value
                state="ctl_bin"

            elif [[ "$arg" == "-S" || "$arg" == "--server-bin" ]]; then
                # Get the next argument as the value
                state="server_bin"

            elif [[ "$arg" == "-s" || "$arg" == "--socket" ]]; then
                # Get the next argument as the value
                state="socket"

            elif [[ "$arg" == "-c" || "$arg" == "--config" ]]; then
                # Get the next argument as the value
                state="config"

            elif [[ "$arg" == '-v' || "$arg" == "--version" ]]; then
                # Get the next argument as the value
                state="version"

            elif [[ "$arg" == "-h" || "$arg" == "--help" ]]; then
                # Show the help string
                echo "Usage: $0 [options] [<mode>]"
                echo ""
                echo "Positionals:"
                echo "  <mode>            Whether to 'install' or 'uninstall'. Default: 'install'"
                echo ""
                echo "Options:"
                echo "  -l,--local-ctl <path>"
                echo "                    If given, uses a local CTL binary instead of downloading"
                echo "                    one."
                echo "  -L,--local-server <path>"
                echo "                    If given, uses a local server binary instead of downloading"
                echo "                    one."
                echo "  -C,--ctl-bin <path>"
                echo "                    Determines the location of the CTL binary. Can be anywhere,"
                echo "                    although we advise it to be somewhere in your PATH. Default:"
                echo "                    '$CTL_BIN'"
                echo "  -S,--server-bin <path>"
                echo "                    Determines the location of the server binary. Can be"
                echo "                    anywhere. Default: '$SERVER_BIN'"
                echo "  -s,--socket <path>"
                echo "                    Determines the location of the CTL/server Unix socket. Can"
                echo "                    be anywhere. Default: '$SOCKET'"
                echo "  -c,--config <path>"
                echo "                    Determines the location of the CTL/server configuration"
                echo "                    file. Can be anywhere, but anything non-default has to be"
                echo "                    passed on every CTL call. Default: '$CONFIG'"
                echo "  -v,--version <version>"
                echo "                    Determines the version of the FileHost to install. Default:"
                echo "                    'latest'"
                echo "  -h,--help         Shows this help string, then quits."
                echo "  --                Ignores any option from the point the double dash occurs on."
                echo ""
                exit 0

            elif [[ "$arg" == "--" ]]; then
                # Do not accept options anymore
                accept_options=0

            else
                echo "Unknown option '$arg'"
                error=1
            fi

        else
            # It's a positional
            if [[ "$pos_i" -eq 0 ]]; then
                # Check the mode
                if [[ "$arg" != "install" && "$arg" != "uninstall" ]]; then
                    echo "Unknown mode '$arg'"
                    error=1
                fi

                # Set it
                mode="$arg"

            else
                echo "Unknown positional '$arg' at index $pos_i"
                error=1
            fi

            # Always increment index
            ((pos_i=pos_i+1))

        fi
    
    elif [[ "$state" == "local_ctl" ]]; then
        # Treat options first
        if [[ accept_options -eq 1 && "$arg" =~ ^- ]]; then
            echo "Missing value for '--local-ctl'"
            error=1
            continue
        fi

        # Grab the value
        local_ctl="$arg"

        # Reset the state
        state="start"
    
    elif [[ "$state" == "local_server" ]]; then
        # Treat options first
        if [[ accept_options -eq 1 && "$arg" =~ ^- ]]; then
            echo "Missing value for '--local-server'"
            error=1
            continue
        fi

        # Grab the value
        local_server="$arg"

        # Reset the state
        state="start"
    
    elif [[ "$state" == "ctl_bin" ]]; then
        # Treat options first
        if [[ accept_options -eq 1 && "$arg" =~ ^- ]]; then
            echo "Missing value for '--ctl-bin'"
            error=1
            continue
        fi

        # Grab the value
        ctl_bin="$arg"

        # Reset the state
        state="start"
    
    elif [[ "$state" == "server_bin" ]]; then
        # Treat options first
        if [[ accept_options -eq 1 && "$arg" =~ ^- ]]; then
            echo "Missing value for '--server-bin'"
            error=1
            continue
        fi

        # Grab the value
        ctl_bin="$arg"

        # Reset the state
        state="start"
    
    elif [[ "$state" == "socket" ]]; then
        # Treat options first
        if [[ accept_options -eq 1 && "$arg" =~ ^- ]]; then
            echo "Missing value for '--socket'"
            error=1
            continue
        fi

        # Grab the value
        socket="$arg"

        # Reset the state
        state="start"
    
    elif [[ "$state" == "config" ]]; then
        # Treat options first
        if [[ accept_options -eq 1 && "$arg" =~ ^- ]]; then
            echo "Missing value for '--config'"
            error=1
            continue
        fi

        # Grab the value
        config="$arg"

        # Reset the state
        state="start"
    
    elif [[ "$state" == "version" ]]; then
        # Treat options first
        if [[ accept_options -eq 1 && "$arg" =~ ^- ]]; then
            echo "Missing value for '--version'"
            error=1
            continue
        fi

        # Match the value with a version number OR latest
        if [[ "$arg" == "latest" ]]; then
            # Set it
            version="$arg"
        elif [[ "$arg" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            # Set it, with 'v'
            version="v$arg"
        else
            echo "Illegal version '$arg'"
            error=1
        fi

        # Reset the state
        state="start"

    else
        echo "ERROR: 'state' cannot be '$state'"
        exit 1
    fi
done

# Quit if we errored
if [[ "$error" -eq 1 ]]; then
    exit 1
fi

# Quit if state is not start
if [[ "$state" == "local_ctl" ]]; then
    echo "Missing value for '--local-ctl'"
    exit 1
elif [[ "$state" == "local_server" ]]; then
    echo "Missing value for '--local-server'"
    exit 1
elif [[ "$state" == "ctl_bin" ]]; then
    echo "Missing value for '--ctl-bin'"
    exit 1
elif [[ "$state" == "server_bin" ]]; then
    echo "Missing value for '--server-bin'"
    exit 1
elif [[ "$state" == "socket" ]]; then
    echo "Missing value for '--socket'"
    exit 1
elif [[ "$state" == "config" ]]; then
    echo "Missing value for '--config'"
    exit 1
elif [[ "$state" == "version" ]]; then
    echo "Missing value for '--version'"
    exit 1
fi





##### OTHER PREPARATION #####
# Try to elevate the script to root
if [[ "$EUID" -ne 0 ]]; then
    # Collect the arguments
    args=""
    for arg in "$@"; do
        # Add either wrapped in quotes or not
        if [[ "$arg" =~ \  ]]; then
            args="$args \"$arg\""
        else
            args="$args $arg"
        fi
    done

    echo "Please run this script as root:"
    echo "$ sudo $0$args"
    exit 1
fi

# Write a nice header
echo ""
echo "*** Setup script for FileHost project ***"
echo "Script version: $SCRIPT_VERSION"
echo "Running for FileHost version: $version"
echo ""





##### INSTALLATION #####
if [[ "$mode" == "install" ]]; then
    echo "Generating folders..."
    echo " > '$(dirname "$socket")'..."
    mkdir -p "$(dirname "$socket")" || exit $?
    echo " > '$(dirname "$config")'..."
    mkdir -p "$(dirname "$config")" || exit $?

    if [ -z "$local_ctl" ]; then
        echo "Downloading $REPOSITORY/$version/filehostctl to '$ctl_bin'..."
        curl --fail -L "$REPOSITORY/$version/filehostctl" --progress-bar -o "$ctl_bin" || exit $?
    else
        echo "Copying '$local_ctl' to '$ctl_bin'..."
        cp "$local_ctl" "$ctl_bin" || exit $?
    fi

    if [ -z "$local_server" ]; then
        echo "Downloading $REPOSITORY/$version/filehostd to '$server_bin'..."
        curl --fail -L "$REPOSITORY/$version/filehostd" --progress-bar -o "$server_bin" || exit $?
    else
        echo "Copying '$local_server' to '$server_bin'..."
        cp "$local_server" "$server_bin" || exit $?
    fi

    echo "Generating '$config'..."
    cat <<EOT > "$config"
{
    "log_level": "DEBUG",
    "socket_path": "$socket",
    "listen_addr": "127.0.0.1:8719",

    "locations": {
        "ctl": "$ctl_bin",
        "server": "$server_bin"
    }
}
EOT

    echo "Creating new user '$USER'..."
    useradd "$USER"
    res=$?
    if [[ "$res" -eq 0 ]]; then
        # Set their password
        passwd "$USER"
    else
        echo " > User already exists"
    fi

    echo "Generating systemd service entry to '$SERVICE_ENTRY'..."
    cat <<EOT > "$SERVICE_ENTRY"
[Unit]
Description=Simple server that serves packages of files that are too large to store in git.
After=network.target filesystemd.socket
Requires=$SOCKET_ENTRY_NAME

[Service]
User=$USER
ExecStart=$server_bin
Restart=always
Environment="CONFIG_PATH=$config"

[Install]
Also=$SERVICE_ENTRY_NAME
WantedBy=multi-user.target
EOT

    echo "Generating systemd socket entry to '$SOCKET_ENTRY'..."
    cat <<EOT > "$SOCKET_ENTRY"
[Unit]
Description=Socket for local CTL communication for the FileHost server.
AssertPathExists=$(dirname "$socket")

[Socket]
ListenDatagram=$socket

[Install]
WantedBy=sockets.target
EOT

    echo "Enabling server in systemd..."
    systemctl enable "$SERVICE_ENTRY_NAME"

    echo "Starting server..."
    systemctl start "$SERVICE_ENTRY_NAME"

    echo ""
    echo "Done."
    echo ""



##### UNINSTALLATION #####
else
    echo "Reading config at '$config' for file locations..."
    ctl_bin=$(python3 -c "import json; h = open(\"$config\", \"r\"); print(json.load(h)[\"locations\"][\"ctl\"]); h.close()") || exit $?
    server_bin=$(python3 -c "import json; h = open(\"$config\", \"r\"); print(json.load(h)[\"locations\"][\"server\"]); h.close()") || exit $?
    socket=$(python3 -c "import json; h = open(\"$config\", \"r\"); print(json.load(h)[\"socket_path\"]); h.close()") || exit $?
    echo " > CTL binary location: $ctl_bin"
    echo " > Server binary location: $server_bin"

    echo "Removing server from systemd..."
    systemctl stop "$SERVICE_ENTRY_NAME"
    systemctl stop "$SOCKET_ENTRY_NAME"
    systemctl disable "$SERVICE_ENTRY_NAME"
    systemctl disable "$SOCKET_ENTRY_NAME"

    echo "Removing systemd socket entry at '$SOCKET_ENTRY'..."
    rm -f "$SOCKET_ENTRY"

    echo "Removing systemd service entry at '$SERVICE_ENTRY'..."
    rm -f "$SERVICE_ENTRY"

    echo "Removing user '$USER'..."
    userdel "$USER"

    echo "Deleting config..."
    rm -f "$config"

    echo "Removing filehostd server binary..."
    rm -f "$server_bin"

    echo "Removing filehostctl binary..."
    rm -f "$ctl_bin"

    echo "Removing directories..."
    echo " > '$(dirname "$config")'..."
    rm -rf "$(dirname "$config")"
    echo " > '$(dirname "$socket")'..."
    rm -rf "$(dirname "$socket")"

    echo ""
    echo "Done."
    echo ""
fi
