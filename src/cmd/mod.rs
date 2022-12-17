/// # 入口指令
/// 命令行管理目录。可以管理维护多个命令行。
/// cmd层负责引导程序启动，显著的工作是初始化逻辑、注册路由对象、启动server监听、阻塞运行程序直至server退出。
pub(crate) mod cmd1;
pub(crate) mod cmd_db;
pub(crate) mod cmd_openapi;
pub(crate) mod cmd_test;
