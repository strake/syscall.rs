/* automatically generated by nr_from_src.py */

pub const _LLSEEK: usize = 236;
pub const _NEWSELECT: usize = 230;
pub const _SYSCTL: usize = 251;
pub const ACCEPT: usize = 99;
pub const ACCEPT4: usize = 323;
pub const ACCESS: usize = 33;
pub const ACCT: usize = 51;
pub const ADD_KEY: usize = 281;
pub const ADJTIMEX: usize = 219;
pub const AFS_SYSCALL: usize = 227;
pub const ALARM: usize = 27;
pub const BDFLUSH: usize = 225;
pub const BIND: usize = 353;
pub const BPF: usize = 349;
pub const BRK: usize = 17;
pub const CAPGET: usize = 21;
pub const CAPSET: usize = 22;
pub const CHDIR: usize = 12;
pub const CHMOD: usize = 15;
pub const CHOWN: usize = 13;
pub const CHOWN32: usize = 35;
pub const CHROOT: usize = 61;
pub const CLOCK_ADJTIME: usize = 334;
pub const CLOCK_GETRES: usize = 258;
pub const CLOCK_GETTIME: usize = 257;
pub const CLOCK_NANOSLEEP: usize = 259;
pub const CLOCK_SETTIME: usize = 256;
pub const CLONE: usize = 217;
pub const CLOSE: usize = 6;
pub const CONNECT: usize = 98;
pub const COPY_FILE_RANGE: usize = 357;
pub const CREAT: usize = 8;
pub const CREATE_MODULE: usize = 221;
pub const DELETE_MODULE: usize = 222;
pub const DUP: usize = 41;
pub const DUP2: usize = 90;
pub const DUP3: usize = 320;
pub const EPOLL_CREATE: usize = 193;
pub const EPOLL_CREATE1: usize = 319;
pub const EPOLL_CTL: usize = 194;
pub const EPOLL_PWAIT: usize = 309;
pub const EPOLL_WAIT: usize = 195;
pub const EVENTFD: usize = 313;
pub const EVENTFD2: usize = 318;
pub const EXECV: usize = 11;
pub const EXECVE: usize = 59;
pub const EXECVEAT: usize = 350;
pub const EXIT: usize = 1;
pub const EXIT_GROUP: usize = 188;
pub const FACCESSAT: usize = 296;
pub const FADVISE64: usize = 209;
pub const FADVISE64_64: usize = 210;
pub const FALLOCATE: usize = 314;
pub const FANOTIFY_INIT: usize = 329;
pub const FANOTIFY_MARK: usize = 330;
pub const FCHDIR: usize = 176;
pub const FCHMOD: usize = 124;
pub const FCHMODAT: usize = 295;
pub const FCHOWN: usize = 123;
pub const FCHOWN32: usize = 32;
pub const FCHOWNAT: usize = 287;
pub const FCNTL: usize = 92;
pub const FCNTL64: usize = 155;
pub const FDATASYNC: usize = 253;
pub const FGETXATTR: usize = 177;
pub const FINIT_MODULE: usize = 342;
pub const FLISTXATTR: usize = 180;
pub const FLOCK: usize = 131;
pub const FORK: usize = 2;
pub const FREMOVEXATTR: usize = 186;
pub const FSETXATTR: usize = 171;
pub const FSTAT: usize = 62;
pub const FSTAT64: usize = 63;
pub const FSTATAT64: usize = 289;
pub const FSTATFS: usize = 158;
pub const FSTATFS64: usize = 235;
pub const FSYNC: usize = 95;
pub const FTRUNCATE: usize = 130;
pub const FTRUNCATE64: usize = 84;
pub const FUTEX: usize = 142;
pub const FUTIMESAT: usize = 288;
pub const GET_KERNEL_SYMS: usize = 223;
pub const GET_MEMPOLICY: usize = 304;
pub const GET_ROBUST_LIST: usize = 301;
pub const GETCPU: usize = 308;
pub const GETCWD: usize = 119;
pub const GETDENTS: usize = 174;
pub const GETDENTS64: usize = 154;
pub const GETDOMAINNAME: usize = 162;
pub const GETEGID: usize = 50;
pub const GETEGID32: usize = 70;
pub const GETEUID: usize = 49;
pub const GETEUID32: usize = 69;
pub const GETGID: usize = 47;
pub const GETGID32: usize = 53;
pub const GETGROUPS: usize = 79;
pub const GETGROUPS32: usize = 115;
pub const GETITIMER: usize = 86;
pub const GETPAGESIZE: usize = 64;
pub const GETPEERNAME: usize = 141;
pub const GETPGID: usize = 224;
pub const GETPGRP: usize = 81;
pub const GETPID: usize = 20;
pub const GETPPID: usize = 197;
pub const GETPRIORITY: usize = 100;
pub const GETRANDOM: usize = 347;
pub const GETRESGID32: usize = 111;
pub const GETRESUID32: usize = 109;
pub const GETRLIMIT: usize = 144;
pub const GETRUSAGE: usize = 117;
pub const GETSID: usize = 252;
pub const GETSOCKNAME: usize = 150;
pub const GETSOCKOPT: usize = 118;
pub const GETTID: usize = 143;
pub const GETTIMEOFDAY: usize = 116;
pub const GETUID: usize = 24;
pub const GETUID32: usize = 44;
pub const GETXATTR: usize = 172;
pub const INIT_MODULE: usize = 190;
pub const INOTIFY_ADD_WATCH: usize = 152;
pub const INOTIFY_INIT: usize = 151;
pub const INOTIFY_INIT1: usize = 322;
pub const INOTIFY_RM_WATCH: usize = 156;
pub const IO_CANCEL: usize = 271;
pub const IO_DESTROY: usize = 269;
pub const IO_GETEVENTS: usize = 272;
pub const IO_SETUP: usize = 268;
pub const IO_SUBMIT: usize = 270;
pub const IOCTL: usize = 54;
pub const IOPRIO_GET: usize = 218;
pub const IOPRIO_SET: usize = 196;
pub const IPC: usize = 215;
pub const KCMP: usize = 341;
pub const KERN_FEATURES: usize = 340;
pub const KEXEC_LOAD: usize = 306;
pub const KEYCTL: usize = 283;
pub const KILL: usize = 37;
pub const LCHOWN: usize = 16;
pub const LCHOWN32: usize = 31;
pub const LGETXATTR: usize = 173;
pub const LINK: usize = 9;
pub const LINKAT: usize = 292;
pub const LISTEN: usize = 354;
pub const LISTXATTR: usize = 178;
pub const LLISTXATTR: usize = 179;
pub const LOOKUP_DCOOKIE: usize = 208;
pub const LREMOVEXATTR: usize = 182;
pub const LSEEK: usize = 19;
pub const LSETXATTR: usize = 170;
pub const LSTAT: usize = 40;
pub const LSTAT64: usize = 132;
pub const MADVISE: usize = 75;
pub const MBIND: usize = 303;
pub const MEMBARRIER: usize = 351;
pub const MEMFD_CREATE: usize = 348;
pub const MIGRATE_PAGES: usize = 302;
pub const MINCORE: usize = 78;
pub const MKDIR: usize = 136;
pub const MKDIRAT: usize = 285;
pub const MKNOD: usize = 14;
pub const MKNODAT: usize = 286;
pub const MLOCK: usize = 237;
pub const MLOCK2: usize = 356;
pub const MLOCKALL: usize = 239;
pub const MMAP: usize = 71;
pub const MMAP2: usize = 56;
pub const MOUNT: usize = 167;
pub const MOVE_PAGES: usize = 307;
pub const MPROTECT: usize = 74;
pub const MQ_GETSETATTR: usize = 278;
pub const MQ_NOTIFY: usize = 277;
pub const MQ_OPEN: usize = 273;
pub const MQ_TIMEDRECEIVE: usize = 276;
pub const MQ_TIMEDSEND: usize = 275;
pub const MQ_UNLINK: usize = 274;
pub const MREMAP: usize = 250;
pub const MSYNC: usize = 65;
pub const MUNLOCK: usize = 238;
pub const MUNLOCKALL: usize = 240;
pub const MUNMAP: usize = 73;
pub const NAME_TO_HANDLE_AT: usize = 332;
pub const NANOSLEEP: usize = 249;
pub const NFSSERVCTL: usize = 254;
pub const NICE: usize = 34;
pub const OLDLSTAT: usize = 202;
pub const OPEN: usize = 5;
pub const OPEN_BY_HANDLE_AT: usize = 333;
pub const OPENAT: usize = 284;
pub const PAUSE: usize = 29;
pub const PCICONFIG_READ: usize = 148;
pub const PCICONFIG_WRITE: usize = 149;
pub const PERF_EVENT_OPEN: usize = 327;
pub const PERFCTR: usize = 18;
pub const PERSONALITY: usize = 191;
pub const PIPE: usize = 42;
pub const PIPE2: usize = 321;
pub const PIVOT_ROOT: usize = 146;
pub const POLL: usize = 153;
pub const PPOLL: usize = 298;
pub const PRCTL: usize = 147;
pub const PREAD64: usize = 67;
pub const PREADV: usize = 324;
pub const PREADV2: usize = 358;
pub const PRLIMIT64: usize = 331;
pub const PROCESS_VM_READV: usize = 338;
pub const PROCESS_VM_WRITEV: usize = 339;
pub const PSELECT6: usize = 297;
pub const PTRACE: usize = 26;
pub const PWRITE64: usize = 68;
pub const PWRITEV: usize = 325;
pub const PWRITEV2: usize = 359;
pub const QUERY_MODULE: usize = 184;
pub const QUOTACTL: usize = 165;
pub const READ: usize = 3;
pub const READAHEAD: usize = 205;
pub const READDIR: usize = 204;
pub const READLINK: usize = 58;
pub const READLINKAT: usize = 294;
pub const READV: usize = 120;
pub const REBOOT: usize = 55;
pub const RECVFROM: usize = 125;
pub const RECVMMSG: usize = 328;
pub const RECVMSG: usize = 113;
pub const REMAP_FILE_PAGES: usize = 192;
pub const REMOVEXATTR: usize = 181;
pub const RENAME: usize = 128;
pub const RENAMEAT: usize = 291;
pub const RENAMEAT2: usize = 345;
pub const REQUEST_KEY: usize = 282;
pub const RESTART_SYSCALL: usize = 0;
pub const RMDIR: usize = 137;
pub const RT_SIGACTION: usize = 102;
pub const RT_SIGPENDING: usize = 104;
pub const RT_SIGPROCMASK: usize = 103;
pub const RT_SIGQUEUEINFO: usize = 106;
pub const RT_SIGRETURN: usize = 101;
pub const RT_SIGSUSPEND: usize = 107;
pub const RT_SIGTIMEDWAIT: usize = 105;
pub const RT_TGSIGQUEUEINFO: usize = 326;
pub const SCHED_GET_AFFINITY: usize = 161;
pub const SCHED_GET_PRIORITY_MAX: usize = 246;
pub const SCHED_GET_PRIORITY_MIN: usize = 247;
pub const SCHED_GETAFFINITY: usize = 260;
pub const SCHED_GETATTR: usize = 344;
pub const SCHED_GETPARAM: usize = 242;
pub const SCHED_GETSCHEDULER: usize = 244;
pub const SCHED_RR_GET_INTERVAL: usize = 248;
pub const SCHED_SET_AFFINITY: usize = 160;
pub const SCHED_SETAFFINITY: usize = 261;
pub const SCHED_SETATTR: usize = 343;
pub const SCHED_SETPARAM: usize = 241;
pub const SCHED_SETSCHEDULER: usize = 243;
pub const SCHED_YIELD: usize = 245;
pub const SECCOMP: usize = 346;
pub const SELECT: usize = 93;
pub const SENDFILE: usize = 39;
pub const SENDFILE64: usize = 140;
pub const SENDMMSG: usize = 336;
pub const SENDMSG: usize = 114;
pub const SENDTO: usize = 133;
pub const SET_MEMPOLICY: usize = 305;
pub const SET_ROBUST_LIST: usize = 300;
pub const SET_TID_ADDRESS: usize = 166;
pub const SETDOMAINNAME: usize = 163;
pub const SETFSGID: usize = 229;
pub const SETFSGID32: usize = 94;
pub const SETFSUID: usize = 228;
pub const SETFSUID32: usize = 91;
pub const SETGID: usize = 46;
pub const SETGID32: usize = 89;
pub const SETGROUPS: usize = 80;
pub const SETGROUPS32: usize = 82;
pub const SETHOSTNAME: usize = 88;
pub const SETITIMER: usize = 83;
pub const SETNS: usize = 337;
pub const SETPGID: usize = 185;
pub const SETPRIORITY: usize = 96;
pub const SETREGID: usize = 127;
pub const SETREGID32: usize = 112;
pub const SETRESGID32: usize = 110;
pub const SETRESUID32: usize = 108;
pub const SETREUID: usize = 126;
pub const SETREUID32: usize = 72;
pub const SETRLIMIT: usize = 145;
pub const SETSID: usize = 175;
pub const SETSOCKOPT: usize = 355;
pub const SETTIMEOFDAY: usize = 122;
pub const SETUID: usize = 23;
pub const SETUID32: usize = 87;
pub const SETXATTR: usize = 169;
pub const SGETMASK: usize = 199;
pub const SHUTDOWN: usize = 134;
pub const SIGACTION: usize = 198;
pub const SIGALTSTACK: usize = 28;
pub const SIGNAL: usize = 48;
pub const SIGNALFD: usize = 311;
pub const SIGNALFD4: usize = 317;
pub const SIGPENDING: usize = 183;
pub const SIGPROCMASK: usize = 220;
pub const SIGRETURN: usize = 216;
pub const SIGSUSPEND: usize = 201;
pub const SOCKET: usize = 97;
pub const SOCKETCALL: usize = 206;
pub const SOCKETPAIR: usize = 135;
pub const SPLICE: usize = 232;
pub const SSETMASK: usize = 200;
pub const STAT: usize = 38;
pub const STAT64: usize = 139;
pub const STATFS: usize = 157;
pub const STATFS64: usize = 234;
pub const STIME: usize = 233;
pub const SWAPOFF: usize = 213;
pub const SWAPON: usize = 85;
pub const SYMLINK: usize = 57;
pub const SYMLINKAT: usize = 293;
pub const SYNC: usize = 36;
pub const SYNC_FILE_RANGE: usize = 255;
pub const SYNCFS: usize = 335;
pub const SYSFS: usize = 226;
pub const SYSINFO: usize = 214;
pub const SYSLOG: usize = 207;
pub const TEE: usize = 280;
pub const TGKILL: usize = 211;
pub const TIME: usize = 231;
pub const TIMER_CREATE: usize = 266;
pub const TIMER_DELETE: usize = 265;
pub const TIMER_GETOVERRUN: usize = 264;
pub const TIMER_GETTIME: usize = 263;
pub const TIMER_SETTIME: usize = 262;
pub const TIMERFD_CREATE: usize = 312;
pub const TIMERFD_GETTIME: usize = 316;
pub const TIMERFD_SETTIME: usize = 315;
pub const TIMES: usize = 43;
pub const TKILL: usize = 187;
pub const TRUNCATE: usize = 129;
pub const TRUNCATE64: usize = 77;
pub const UMASK: usize = 60;
pub const UMOUNT: usize = 159;
pub const UMOUNT2: usize = 45;
pub const UNAME: usize = 189;
pub const UNLINK: usize = 10;
pub const UNLINKAT: usize = 290;
pub const UNSHARE: usize = 299;
pub const USELIB: usize = 203;
pub const USERFAULTFD: usize = 352;
pub const USTAT: usize = 168;
pub const UTIME: usize = 30;
pub const UTIMENSAT: usize = 310;
pub const UTIMES: usize = 138;
pub const VFORK: usize = 66;
pub const VHANGUP: usize = 76;
pub const VMSPLICE: usize = 25;
pub const WAIT4: usize = 7;
pub const WAITID: usize = 279;
pub const WAITPID: usize = 212;
pub const WRITE: usize = 4;
pub const WRITEV: usize = 121;
