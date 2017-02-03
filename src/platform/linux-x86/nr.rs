/* Automatically generated by sc-gen 0.1.0 */

pub const _LLSEEK: usize = 140;
pub const _NEWSELECT: usize = 142;
pub const _SYSCTL: usize = 149;
// pub const ACCEPT: usize = __NR_accept;
// pub const ACCEPT4: usize = __NR_accept4;
pub const ACCESS: usize = 33;
pub const ACCT: usize = 51;
pub const ADD_KEY: usize = 286;
pub const ADJTIMEX: usize = 124;
pub const ALARM: usize = 27;
// pub const ARM_FADVISE64_64: usize = __NR_arm_fadvise64_64;
// pub const ARM_SYNC_FILE_RANGE: usize = __NR_arm_sync_file_range;
pub const BDFLUSH: usize = 134;
// pub const BIND: usize = __NR_bind;
pub const BRK: usize = 45;
pub const CAPGET: usize = 184;
pub const CAPSET: usize = 185;
pub const CHDIR: usize = 12;
pub const CHMOD: usize = 15;
pub const CHOWN: usize = 182;
pub const CHOWN32: usize = 212;
pub const CHROOT: usize = 61;
pub const CLOCK_ADJTIME: usize = 343;
pub const CLOCK_GETRES: usize = 266;
pub const CLOCK_GETTIME: usize = 265;
pub const CLOCK_NANOSLEEP: usize = 267;
pub const CLOCK_SETTIME: usize = 264;
pub const CLONE: usize = 120;
pub const CLOSE: usize = 6;
// pub const CONNECT: usize = __NR_connect;
pub const CREAT: usize = 8;
pub const DELETE_MODULE: usize = 129;
pub const DUP: usize = 41;
pub const DUP2: usize = 63;
pub const DUP3: usize = 330;
pub const EPOLL_CREATE: usize = 254;
pub const EPOLL_CREATE1: usize = 329;
pub const EPOLL_CTL: usize = 255;
pub const EPOLL_PWAIT: usize = 319;
pub const EPOLL_WAIT: usize = 256;
pub const EVENTFD: usize = 323;
pub const EVENTFD2: usize = 328;
pub const EXECVE: usize = 11;
pub const EXIT: usize = 1;
pub const EXIT_GROUP: usize = 252;
pub const FACCESSAT: usize = 307;
pub const FALLOCATE: usize = 324;
pub const FANOTIFY_INIT: usize = 338;
pub const FANOTIFY_MARK: usize = 339;
pub const FCHDIR: usize = 133;
pub const FCHMOD: usize = 94;
pub const FCHMODAT: usize = 306;
pub const FCHOWN: usize = 95;
pub const FCHOWN32: usize = 207;
pub const FCHOWNAT: usize = 298;
pub const FCNTL: usize = 55;
pub const FCNTL64: usize = 221;
pub const FDATASYNC: usize = 148;
pub const FGETXATTR: usize = 231;
// pub const FINIT_MODULE: usize = __NR_finit_module;
pub const FLISTXATTR: usize = 234;
pub const FLOCK: usize = 143;
pub const FORK: usize = 2;
pub const FREMOVEXATTR: usize = 237;
pub const FSETXATTR: usize = 228;
pub const FSTAT: usize = 108;
pub const FSTAT64: usize = 197;
pub const FSTATAT64: usize = 300;
pub const FSTATFS: usize = 100;
pub const FSTATFS64: usize = 269;
pub const FSYNC: usize = 118;
pub const FTRUNCATE: usize = 93;
pub const FTRUNCATE64: usize = 194;
pub const FUTEX: usize = 240;
pub const FUTIMESAT: usize = 299;
pub const GET_MEMPOLICY: usize = 275;
pub const GET_ROBUST_LIST: usize = 312;
pub const GETCPU: usize = 318;
pub const GETCWD: usize = 183;
pub const GETDENTS: usize = 141;
pub const GETDENTS64: usize = 220;
pub const GETEGID: usize = 50;
pub const GETEGID32: usize = 202;
pub const GETEUID: usize = 49;
pub const GETEUID32: usize = 201;
pub const GETGID: usize = 47;
pub const GETGID32: usize = 200;
pub const GETGROUPS: usize = 80;
pub const GETGROUPS32: usize = 205;
pub const GETITIMER: usize = 105;
// pub const GETPEERNAME: usize = __NR_getpeername;
pub const GETPGID: usize = 132;
pub const GETPGRP: usize = 65;
pub const GETPID: usize = 20;
pub const GETPPID: usize = 64;
pub const GETPRIORITY: usize = 96;
pub const GETRANDOM: usize = 355;
pub const GETRESGID: usize = 171;
pub const GETRESGID32: usize = 211;
pub const GETRESUID: usize = 165;
pub const GETRESUID32: usize = 209;
pub const GETRLIMIT: usize = 76;
pub const GETRUSAGE: usize = 77;
pub const GETSID: usize = 147;
// pub const GETSOCKNAME: usize = __NR_getsockname;
// pub const GETSOCKOPT: usize = __NR_getsockopt;
pub const GETTID: usize = 224;
pub const GETTIMEOFDAY: usize = 78;
pub const GETUID: usize = 24;
pub const GETUID32: usize = 199;
pub const GETXATTR: usize = 229;
pub const INIT_MODULE: usize = 128;
pub const INOTIFY_ADD_WATCH: usize = 292;
pub const INOTIFY_INIT: usize = 291;
pub const INOTIFY_INIT1: usize = 332;
pub const INOTIFY_RM_WATCH: usize = 293;
pub const IO_CANCEL: usize = 249;
pub const IO_DESTROY: usize = 246;
pub const IO_GETEVENTS: usize = 247;
pub const IO_SETUP: usize = 245;
pub const IO_SUBMIT: usize = 248;
pub const IOCTL: usize = 54;
pub const IOPRIO_GET: usize = 290;
pub const IOPRIO_SET: usize = 289;
pub const IPC: usize = 117;
// pub const KCMP: usize = __NR_kcmp;
pub const KEXEC_LOAD: usize = 283;
pub const KEYCTL: usize = 288;
pub const KILL: usize = 37;
pub const LCHOWN: usize = 16;
pub const LCHOWN32: usize = 198;
pub const LGETXATTR: usize = 230;
pub const LINK: usize = 9;
pub const LINKAT: usize = 303;
// pub const LISTEN: usize = __NR_listen;
pub const LISTXATTR: usize = 232;
pub const LLISTXATTR: usize = 233;
pub const LOOKUP_DCOOKIE: usize = 253;
pub const LREMOVEXATTR: usize = 236;
pub const LSEEK: usize = 19;
pub const LSETXATTR: usize = 227;
pub const LSTAT: usize = 107;
pub const LSTAT64: usize = 196;
pub const MADVISE: usize = 219;
pub const MBIND: usize = 274;
pub const MINCORE: usize = 218;
pub const MKDIR: usize = 39;
pub const MKDIRAT: usize = 296;
pub const MKNOD: usize = 14;
pub const MKNODAT: usize = 297;
pub const MLOCK: usize = 150;
pub const MLOCKALL: usize = 152;
pub const MMAP: usize = 90;
pub const MMAP2: usize = 192;
pub const MOUNT: usize = 21;
pub const MOVE_PAGES: usize = 317;
pub const MPROTECT: usize = 125;
pub const MQ_GETSETATTR: usize = 282;
pub const MQ_NOTIFY: usize = 281;
pub const MQ_OPEN: usize = 277;
pub const MQ_TIMEDRECEIVE: usize = 280;
pub const MQ_TIMEDSEND: usize = 279;
pub const MQ_UNLINK: usize = 278;
pub const MREMAP: usize = 163;
// pub const MSGCTL: usize = __NR_msgctl;
// pub const MSGGET: usize = __NR_msgget;
// pub const MSGRCV: usize = __NR_msgrcv;
// pub const MSGSND: usize = __NR_msgsnd;
pub const MSYNC: usize = 144;
pub const MUNLOCK: usize = 151;
pub const MUNLOCKALL: usize = 153;
pub const MUNMAP: usize = 91;
pub const NAME_TO_HANDLE_AT: usize = 341;
pub const NANOSLEEP: usize = 162;
// pub const NEWFSTATAT: usize = __NR_newfstatat;
pub const NFSSERVCTL: usize = 169;
pub const NICE: usize = 34;
pub const OPEN: usize = 5;
pub const OPEN_BY_HANDLE_AT: usize = 342;
pub const OPENAT: usize = 295;
pub const PAUSE: usize = 29;
// pub const PCICONFIG_IOBASE: usize = __NR_pciconfig_iobase;
// pub const PCICONFIG_READ: usize = __NR_pciconfig_read;
// pub const PCICONFIG_WRITE: usize = __NR_pciconfig_write;
pub const PERF_EVENT_OPEN: usize = 336;
pub const PERSONALITY: usize = 136;
pub const PIPE: usize = 42;
pub const PIPE2: usize = 331;
pub const PIVOT_ROOT: usize = 217;
pub const POLL: usize = 168;
pub const PPOLL: usize = 309;
pub const PRCTL: usize = 172;
pub const PREAD64: usize = 180;
pub const PREADV: usize = 333;
pub const PRLIMIT64: usize = 340;
pub const PROCESS_VM_READV: usize = 347;
pub const PROCESS_VM_WRITEV: usize = 348;
pub const PSELECT6: usize = 308;
pub const PTRACE: usize = 26;
pub const PWRITE64: usize = 181;
pub const PWRITEV: usize = 334;
pub const QUOTACTL: usize = 131;
pub const READ: usize = 3;
pub const READAHEAD: usize = 225;
pub const READDIR: usize = 89;
pub const READLINK: usize = 85;
pub const READLINKAT: usize = 305;
pub const READV: usize = 145;
pub const REBOOT: usize = 88;
// pub const RECV: usize = __NR_recv;
// pub const RECVFROM: usize = __NR_recvfrom;
pub const RECVMMSG: usize = 337;
// pub const RECVMSG: usize = __NR_recvmsg;
pub const REMAP_FILE_PAGES: usize = 257;
pub const REMOVEXATTR: usize = 235;
pub const RENAME: usize = 38;
pub const RENAMEAT: usize = 302;
pub const REQUEST_KEY: usize = 287;
pub const RESTART_SYSCALL: usize = 0;
pub const RMDIR: usize = 40;
pub const RT_SIGACTION: usize = 174;
pub const RT_SIGPENDING: usize = 176;
pub const RT_SIGPROCMASK: usize = 175;
pub const RT_SIGQUEUEINFO: usize = 178;
pub const RT_SIGRETURN: usize = 173;
pub const RT_SIGSUSPEND: usize = 179;
pub const RT_SIGTIMEDWAIT: usize = 177;
pub const RT_TGSIGQUEUEINFO: usize = 335;
pub const SCHED_GET_PRIORITY_MAX: usize = 159;
pub const SCHED_GET_PRIORITY_MIN: usize = 160;
pub const SCHED_GETAFFINITY: usize = 242;
pub const SCHED_GETPARAM: usize = 155;
pub const SCHED_GETSCHEDULER: usize = 157;
pub const SCHED_RR_GET_INTERVAL: usize = 161;
pub const SCHED_SETAFFINITY: usize = 241;
pub const SCHED_SETPARAM: usize = 154;
pub const SCHED_SETSCHEDULER: usize = 156;
pub const SCHED_YIELD: usize = 158;
pub const SELECT: usize = 82;
// pub const SEMCTL: usize = __NR_semctl;
// pub const SEMGET: usize = __NR_semget;
// pub const SEMOP: usize = __NR_semop;
// pub const SEMTIMEDOP: usize = __NR_semtimedop;
// pub const SEND: usize = __NR_send;
pub const SENDFILE: usize = 187;
pub const SENDFILE64: usize = 239;
pub const SENDMMSG: usize = 345;
// pub const SENDMSG: usize = __NR_sendmsg;
// pub const SENDTO: usize = __NR_sendto;
pub const SET_MEMPOLICY: usize = 276;
pub const SET_ROBUST_LIST: usize = 311;
pub const SET_TID_ADDRESS: usize = 258;
pub const SETDOMAINNAME: usize = 121;
pub const SETFSGID: usize = 139;
pub const SETFSGID32: usize = 216;
pub const SETFSUID: usize = 138;
pub const SETFSUID32: usize = 215;
pub const SETGID: usize = 46;
pub const SETGID32: usize = 214;
pub const SETGROUPS: usize = 81;
pub const SETGROUPS32: usize = 206;
pub const SETHOSTNAME: usize = 74;
pub const SETITIMER: usize = 104;
pub const SETNS: usize = 346;
pub const SETPGID: usize = 57;
pub const SETPRIORITY: usize = 97;
pub const SETREGID: usize = 71;
pub const SETREGID32: usize = 204;
pub const SETRESGID: usize = 170;
pub const SETRESGID32: usize = 210;
pub const SETRESUID: usize = 164;
pub const SETRESUID32: usize = 208;
pub const SETREUID: usize = 70;
pub const SETREUID32: usize = 203;
pub const SETRLIMIT: usize = 75;
pub const SETSID: usize = 66;
// pub const SETSOCKOPT: usize = __NR_setsockopt;
pub const SETTIMEOFDAY: usize = 79;
pub const SETUID: usize = 23;
pub const SETUID32: usize = 213;
pub const SETXATTR: usize = 226;
// pub const SHMAT: usize = __NR_shmat;
// pub const SHMCTL: usize = __NR_shmctl;
// pub const SHMDT: usize = __NR_shmdt;
// pub const SHMGET: usize = __NR_shmget;
// pub const SHUTDOWN: usize = __NR_shutdown;
pub const SIGACTION: usize = 67;
pub const SIGALTSTACK: usize = 186;
pub const SIGNALFD: usize = 321;
pub const SIGNALFD4: usize = 327;
pub const SIGPENDING: usize = 73;
pub const SIGPROCMASK: usize = 126;
pub const SIGRETURN: usize = 119;
pub const SIGSUSPEND: usize = 72;
// pub const SOCKET: usize = __NR_socket;
pub const SOCKETCALL: usize = 102;
// pub const SOCKETPAIR: usize = __NR_socketpair;
pub const SPLICE: usize = 313;
pub const STAT: usize = 106;
pub const STAT64: usize = 195;
pub const STATFS: usize = 99;
pub const STATFS64: usize = 268;
pub const STIME: usize = 25;
pub const SWAPOFF: usize = 115;
pub const SWAPON: usize = 87;
pub const SYMLINK: usize = 83;
pub const SYMLINKAT: usize = 304;
pub const SYNC: usize = 36;
// pub const SYNC_FILE_RANGE2: usize = __NR_sync_file_range2;
pub const SYNCFS: usize = 344;
// pub const SYSCALL: usize = __NR_syscall;
pub const SYSFS: usize = 135;
pub const SYSINFO: usize = 116;
pub const SYSLOG: usize = 103;
pub const TEE: usize = 315;
pub const TGKILL: usize = 270;
pub const TIME: usize = 13;
pub const TIMER_CREATE: usize = 259;
pub const TIMER_DELETE: usize = 263;
pub const TIMER_GETOVERRUN: usize = 262;
pub const TIMER_GETTIME: usize = 261;
pub const TIMER_SETTIME: usize = 260;
pub const TIMERFD_CREATE: usize = 322;
pub const TIMERFD_GETTIME: usize = 326;
pub const TIMERFD_SETTIME: usize = 325;
pub const TIMES: usize = 43;
pub const TKILL: usize = 238;
pub const TRUNCATE: usize = 92;
pub const TRUNCATE64: usize = 193;
pub const UGETRLIMIT: usize = 191;
pub const UMASK: usize = 60;
pub const UMOUNT: usize = 22;
pub const UMOUNT2: usize = 52;
pub const UNAME: usize = 122;
pub const UNLINK: usize = 10;
pub const UNLINKAT: usize = 301;
pub const UNSHARE: usize = 310;
pub const USELIB: usize = 86;
pub const USTAT: usize = 62;
pub const UTIME: usize = 30;
pub const UTIMENSAT: usize = 320;
pub const UTIMES: usize = 271;
pub const VFORK: usize = 190;
pub const VHANGUP: usize = 111;
pub const VMSPLICE: usize = 316;
pub const VSERVER: usize = 273;
pub const WAIT4: usize = 114;
pub const WAITID: usize = 284;
pub const WRITE: usize = 4;
pub const WRITEV: usize = 146;
