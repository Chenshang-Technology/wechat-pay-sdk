use serde::Deserialize;

/// 微信支付接口错误码
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum WeChatPayApiErrorCode {
  Common(Common),
  Order(Order),
  Refund(Refund),
}

/// [公共错误码](https://pay.weixin.qq.com/wiki/doc/apiv3/Share/error_code.shtml)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Common {
  /// 商户号与 appid 不匹配
  ///
  /// 请绑定调用接口的商户号和APPID后重试
  AppidMchidNotMatch,
  /// 银行系统异常
  ///
  /// 银行系统异常，请用相同参数重新调用
  BankError,
  /// 商户订单号重复
  ///
  /// 请核实商户订单号是否重复提交
  OutTradeNoUsed,
  /// 请求受阻
  ///
  /// 此状态代表退款申请失败，商户可根据具体的错误提示做相应的处理
  RequestBlocked,
  /// 退款业务流程错误
  ///
  /// 请不要更换商户退款单号，请使用相同参数再次调用 API
  BizErrNeedRetry,
  /// 用户支付中，需要输入密码
  ///
  /// 等待5秒，然后调用被扫订单结果查询 API，查询当前订单的不同状态，决定下一步的操作
  Userpaying,
  /// 参数错误
  ///
  /// 根据错误提示，传入正确参数
  ParamError,
  /// 请求的资源不存在
  ///
  /// 请商户检查需要查询的 id 或者请求 URL 是否正确
  OrderNotExist,
  /// 签约协议不存在
  ///
  /// 请检查签约协议号是否正确，是否已解约
  ContractNotExist,
  /// 手机号不存在
  ///
  /// 请检查手机号码是否正确
  PhoneNotExist,
  /// 签名验证失败
  ///
  /// 请检查签名参数和方法是否都符合签名算法要求
  SignError,
  /// 账号异常
  ///
  /// 用户账号异常，无需更多操作
  AccountError,
  /// 系统错误
  ///
  /// 5 开头的状态码都为系统问题，请使用相同参数 稍后重新调用
  SystemError,
  /// 收银员扫描的不是微信支付的条码
  ///
  /// 请扫描微信支付被扫条码 / 二维码
  AuthCodeInvalid,
  /// 频率超限
  ///
  /// 请求量不要超过接口调用频率限制
  FrequencyLimited,
  /// 频率限制
  ///
  /// 请降低频率后重试
  RatelimitExceeded,
  /// 商户暂无权限使用此功能
  ///
  /// 请开通商户号权限。请联系产品或商务申请
  NoAuth,
  /// 业务规则限制
  ///
  /// 因业务规则限制请求频率，请查看接口返回的详细信息
  RuleLimit,
  /// 用户的条码已经过期
  ///
  /// 请收银员提示用户，请用户在微信上刷新条码，然后请收银员重新扫码。 直接将错误展示给收银员
  AuthCodeExpire,
  /// 交易错误
  ///
  /// 因业务原因交易失败，请查看接口返回的详细信息
  TradeError,
  /// 用户账户注销
  ///
  /// 请检查用户账户是否正确
  UserNotExist,
  /// 业务错误
  ///
  /// 该错误都会返回具体的错误原因，请根据实际返回做相应处理
  Error,
  /// 接口限频
  ///
  /// 请降低调用频率
  FrequencyLimitExceed,
  /// 协议已存在
  ///
  /// 已开通自动扣费服务功能，无需重复开通
  ContractExisted,
  /// 用户账户异常
  ///
  /// 该确认用户账号是否正常，商家可联系微信支付或让用户联系微信支付客服处理
  UserAccountAbnormal,
  /// 当前用户签约状态失效
  ///
  /// 请通过查询用户接口核实签约状态
  ContractError,
  /// 订单号错误或订单状态不正确
  ///
  /// 请检查订单号是否有误以及订单状态是否正确，如：未支付、已支付未退款
  RefundNotExists,
  /// 二级商户未开启手动提现权限
  ///
  /// 二级商户号提现权限已关闭，无法发起提现
  ContractNotConfirmed,
  /// 账单文件不存在
  ///
  /// 请检查当前商户号是否在指定日期有交易或退款发生
  NoStatementExist,
  /// 账单生成中
  ///
  /// 请先检查当前商户号在指定日期内是否有成功的交易或退款，若有，则在T+1日上午8点后再重新下载
  StatementCreating,
  /// 商户号不存在
  ///
  /// 请确认传入的商户号是否正确
  MchNotExists,
  /// 请求参数符合参数格式，但不符合业务规则
  ///
  /// 请确认相同单号是否使用了不同的参数
  InvalidRequest,
  /// 查询的资源不存在
  ///
  /// 请检查查询资源的对应id是否填写正确
  ResourceNotExists,
  /// 用户已签约该商户，不可重复签约
  ///
  /// 请通过查询用户接口获取用户的签约信息
  ResourceAlreadyExists,
  /// 资源已存在
  ///
  /// 尝试创建的资源已存在，无需重复创建
  AlreadyExists,
  /// 服务未开通或账号未注册
  ///
  /// 该用户尚未注册或开通当前服务，请开通后再试
  UserNotRegistered,
  /// openid 不正确
  ///
  /// 请确认传入的 openid 是否正确
  UserNotExists,
  /// 订单已关闭
  ///
  /// 当前订单已关闭，请重新下单
  OrderClosed,
  /// 订单已支付
  ///
  /// 请确认该订单号是否重复支付，如果是新单，请使用新订单号提交
  OrderPaid,
  /// 订单已撤销
  ///
  /// 当前订单状态为“订单已撤销”，请提示用户重新支付
  OrderReversed,
  /// 订单已关闭
  ///
  /// 商户订单号异常，请重新下单支付
  Orderclosed,
  /// 订单已支付
  ///
  /// 请确认该订单号是否重复支付，如果是新单，请使用新订单号提交
  Orderpaid,
  /// 订单已撤销
  ///
  /// 当前订单状态为“订单已撤销”，请提示用户重新支付
  Orderreversed,
  /// 二级商户下行打款未成功
  ///
  /// 二级商户号结算银行卡信息有误，修改后重试
  AccountNotVerified,
  /// 请求的资源不存在
  ///
  /// 请商户检查需要查询的 id 或者请求 URL 是否正确
  NotFound,
}

/// 下单错误：
/// - [JSApi 下单](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_1.shtml)
/// - [App 下单](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_2_1.shtml)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Order {
  /// openid和appid不匹配
  ///
  /// 请确认openid和appid是否匹配
  OpenidMismatch,
  /// 订单号非法
  ///
  /// 请检查微信支付订单号是否正确
  InvalidTransactionid,
}

/// 退款错误
/// - [申请退款](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_9.shtml)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Refund {
  /// 余额不足
  ///
  /// 此状态代表退款申请失败，商户可根据具体的错误提示做相应的处理。
  NotEnough,
}
