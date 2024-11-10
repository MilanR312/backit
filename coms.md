
start
reload
stop

# definitions
type <credentials> = <p2pid> <pw> | -k <key> | -u <url>
> defines how to connect to a remote host
type <host_id> = <host_nickname> | <p2pid>
> defines how we identify a remote host
type <FileTarget> = <file> [-n nickname] | -r <dir>
> defines how we add new files to host
type <Target> = <nickname> | -t <tag>,+
> defines how we search for files on a host
type <AnyHost> = <host_id> | -c <credentials>
# connection related commands

connect <credentials> [-n <host_nickname>]
disconnect <host_id>

# hosting & fetching related commands

host <FileTarget> [-t <tag>,+]
> host a file or directory, add an optional nickname if its a file and add every tag of the optional taglist

unhost <Target>' '+
> unhost all items of the list
>   - when file it unhosts all files of the list
>   - when nickname it unhosts all files with nickname
>   - when a tag it unhosts all files that match all of the items in the list

fetch <AnyHost> <Target>' '+
> fetch all items of the list
> see unhost

push <AnyHost> <Target>' '+
> push all items

backup <AnyHost> <Target> [-c <compressiontype>] [-s <schedule>]
> allows compression and schedule backups [Once|Hourly|Dayly|Weekly|Monthly|CustomTime]

# info related commands
info [-h <AnyHost>]
> get info for the local host or remote host

filelist [-h <AnyHost>]
> get the list of files for the local host or remote host




