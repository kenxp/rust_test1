/// # 接口处理
/// 接收/解析用户输入参数的入口/接口层。
/// - controller用于接收api的输入，调用内部的一个或多个service包实现业务场景，组织service的结果构造为api的输出数据结构。
/// - controller层负责接收Req请求对象后做一些业务逻辑校验，随后调用一个或多个service实现业务逻辑，将执行结构封装为约定的Res数据结构对象返回。
/// - controller层处理Req/Res外部接口请求。负责接收、校验请求参数，并调用一个或多个 service来实现业务逻辑处理，根据返回数据结构组装数据再返回。service层处理Input/Output内部方法调用。负责内部可复用的业务逻辑封装，封装的方法粒度往往比较细。
pub mod auth;
