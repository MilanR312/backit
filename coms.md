
start
reload
stop

# connection related commands

connect <id> <pw> [-n <nick>]
> connect permanently to a remote host and set a nickname if supplied
connect <id> -k <shared_key> [-n <nick>]
> connect permanently to a remote host via a shared_key

disconect [<id|nick>] [--json]
> disconnect from a remote host via id or nickname if supplied
> if neither is supplied open a menu, if the json flag is supplied the menu is not shown and a json output of all connected hosts is supplied

# hosting & fetching related commands

host <file> [-n <nickname>]
> host a file for all connected hosts
host -r <dir> [-n <nickname>]
> host a directory for all connected hosts

unhost <DIR|FILE|NICKNAME>
> unhost a file or directory for all connected hosts


fetch <HOSTID|NICKNAME> <DIR|FILE|NICKNAME>
> fetch a hosted file from the host

fetch <HOSTID|NICKNAME> -b <BACKUPID>
> fetch a backup from the host

fetch <HOSTID> -p <PASSWORD>
fetch <HOSTID> -k <SESSIONKEY>

push <DIR|FILE> <HOSTID|NICKNAME>
push <DIR|FILE> <HOSTID> -p <PASSWORD>
push <DIR|FILE> <HOSTID> -K <SESSIONKEY>


backup <path> [-c <compressiontype>] [-s <schedule>] <HOSTID|NICKNAME>
> backup a file to the remote host, the host must have backups enabled for this
> allows compression and schedule backups [Once|Hourly|Dayly|Weekly|Monthly|CustomTime]

# info related commands
info [-h <HOSTID|NICKNAME>]
> get info for the local host or remote host

filelist [-h <HOSTID|NICKNAME>]
> get the list of files for the local host or remote host






# needs connection
file copy <path> [<nickname>|<hostid>] <path>
file fetch [<nickname>|<hostid>] <path> <path>
