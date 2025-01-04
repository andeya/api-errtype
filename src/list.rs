macro_rules! const_errtype {
    ($err_type_ident:ident, $err_type_method:ident, $chinese_desc:expr, $english_desc:expr, $explanation:expr) => {
        #[doc = $explanation]
        pub const $err_type_ident: api_response::error_code::ErrType =
            api_response::error_code::ErrType::$err_type_method(if cfg!(feature = "chinese_description") {
                $chinese_desc
            } else {
                $english_desc
            });
    };
}

const_errtype!(
    ET_USER,
    T1000,
    "用户端错误",
    "User Error",
    "Primary macroscopic error code"
);

const_errtype!(
    ET_USER_EQUIPMENT,
    T1001,
    "用户设备异常",
    "User Equipment Error",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_USER_CAMERA,
    T1002,
    "用户相机异常",
    "User Camera Error",
    "Specific case under user equipment error"
);

const_errtype!(
    ET_USER_MICROPHONE,
    T1003,
    "用户麦克风异常",
    "User Microphone Error",
    "Specific case under user equipment error"
);

const_errtype!(
    ET_USER_EARPIECE,
    T1004,
    "用户听筒异常",
    "User Earpiece Error",
    "Specific case under user equipment error"
);

const_errtype!(
    ET_USER_SPEAKER,
    T1005,
    "用户扬声器异常",
    "User Speaker Error",
    "Specific case under user equipment error"
);

const_errtype!(
    ET_USER_GPS,
    T1006,
    "用户GPS定位异常",
    "User GPS Error",
    "Specific case under user equipment error"
);

const_errtype!(
    ET_BUSINESS_SERVICE,
    T2000,
    "业务服务错误",
    "Business Service Error",
    "Primary macroscopic error code"
);

const_errtype!(
    ET_USER_REGISTRATION,
    T2001,
    "用户注册错误",
    "User Registration Error",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_USER_PRIVACY_POLICY,
    T2002,
    "用户未同意隐私协议",
    "User Did Not Agree To Privacy Policy",
    "Specific case under user registration error"
);

const_errtype!(
    ET_REGISTRATION_RESTRICTED,
    T2003,
    "注册国家或地区受限",
    "Registration Country or Region Restricted",
    "Specific case under user registration error"
);

const_errtype!(
    ET_USERNAME_VERIFICATION_FAILED,
    T2004,
    "用户名校验失败",
    "Username Verification Failed",
    "Specific case under user registration error"
);

const_errtype!(
    ET_USERNAME_EXISTS,
    T2005,
    "用户名已存在",
    "Username Already Exists",
    "Specific case under user registration error"
);

const_errtype!(
    ET_USERNAME_SENSITIVE_WORDS,
    T2006,
    "用户名包含敏感词",
    "Username Contains Sensitive Words",
    "Specific case under user registration error"
);

const_errtype!(
    ET_USERNAME_SPECIAL_CHARACTERS,
    T2007,
    "用户名包含特殊字符",
    "Username Contains Special Characters",
    "Specific case under user registration error"
);

const_errtype!(
    ET_PASSWORD_VERIFICATION_FAILED,
    T2008,
    "密码校验失败",
    "Password Verification Failed",
    "Specific case under user registration error"
);

const_errtype!(
    ET_PASSWORD_TOO_SHORT,
    T2009,
    "密码长度不够",
    "Password Too Short",
    "Specific case under user registration error"
);

const_errtype!(
    ET_PASSWORD_STRENGTH_INSUFFICIENT,
    T2010,
    "密码强度不够",
    "Password Strength Insufficient",
    "Specific case under user registration error"
);

const_errtype!(
    ET_VERIFICATION_CODE,
    T2011,
    "校验码输入错误",
    "Verification Code Error",
    "Specific case under user registration error"
);

const_errtype!(
    ET_SMS_VERIFICATION_CODE,
    T2012,
    "短信校验码输入错误",
    "SMS Verification Code Error",
    "Specific case under verification code error"
);

const_errtype!(
    ET_EMAIL_VERIFICATION_CODE,
    T2013,
    "邮件校验码输入错误",
    "Email Verification Code Error",
    "Specific case under verification code error"
);

const_errtype!(
    ET_VOICE_VERIFICATION_CODE,
    T2014,
    "语音校验码输入错误",
    "Voice Verification Code Error",
    "Specific case under verification code error"
);

const_errtype!(
    ET_USER_DOCUMENT,
    T2015,
    "用户证件异常",
    "User Document Error",
    "Specific case under user registration error"
);

const_errtype!(
    ET_USER_DOCUMENT_TYPE_NOT_SELECTED,
    T2016,
    "用户证件类型未选择",
    "User Document Type Not Selected",
    "Specific case under user document error"
);

const_errtype!(
    ET_MAINLAND_ID_VERIFICATION_ILLEGAL,
    T2017,
    "大陆身份证编号校验非法",
    "Mainland ID Number Verification Illegal",
    "Specific case under user document error"
);

const_errtype!(
    ET_PASSPORT_VERIFICATION_ILLEGAL,
    T2018,
    "护照编号校验非法",
    "Passport Number Verification Illegal",
    "Specific case under user document error"
);

const_errtype!(
    ET_OFFICER_ID_VERIFICATION_ILLEGAL,
    T2019,
    "军官证编号校验非法",
    "Officer ID Number Verification Illegal",
    "Specific case under user document error"
);

const_errtype!(
    ET_USER_BASIC_INFO_VERIFICATION_FAILED,
    T2020,
    "用户基本信息校验失败",
    "User Basic Information Verification Failed",
    "Specific case under user registration error"
);

const_errtype!(
    ET_PHONE_VERIFICATION_FAILED,
    T2021,
    "手机格式校验失败",
    "Phone Format Verification Failed",
    "Specific case under user basic information verification failed"
);

const_errtype!(
    ET_ADDRESS_VERIFICATION_FAILED,
    T2022,
    "地址格式校验失败",
    "Address Format Verification Failed",
    "Specific case under user basic information verification failed"
);

const_errtype!(
    ET_EMAIL_VERIFICATION_FAILED,
    T2023,
    "邮箱格式校验失败",
    "Email Format Verification Failed",
    "Specific case under user basic information verification failed"
);

const_errtype!(
    ET_USER_LOGIN,
    T2024,
    "用户登录异常",
    "User Login Error",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_USER_ACCOUNT_DOES_NOT_EXIST,
    T2025,
    "用户账户不存在",
    "User Account Does Not Exist",
    "Specific case under user login error"
);

const_errtype!(
    ET_USER_ACCOUNT_FROZEN,
    T2026,
    "用户账户被冻结",
    "User Account Frozen",
    "Specific case under user login error"
);

const_errtype!(
    ET_USER_ACCOUNT_INVALIDATED,
    T2027,
    "用户账户已作废",
    "User Account Invalidated",
    "Specific case under user login error"
);

const_errtype!(
    ET_USER_PASSWORD,
    T2028,
    "用户密码错误",
    "User Password Error",
    "Specific case under user login error"
);

const_errtype!(
    ET_USER_PASSWORD_LIMIT_EXCEEDED,
    T2029,
    "用户输入密码错误次数超限",
    "User Exceeded Password Error Limit",
    "Specific case under user password error"
);

const_errtype!(
    ET_USER_IDENTITY_VERIFICATION_FAILED,
    T2030,
    "用户身份校验失败",
    "User Identity Verification Failed",
    "Specific case under user login error"
);

const_errtype!(
    ET_USER_FINGERPRINT_VERIFICATION_FAILED,
    T2031,
    "用户指纹识别失败",
    "User Fingerprint Verification Failed",
    "Specific case under user identity verification failed"
);

const_errtype!(
    ET_USER_FACE_RECOGNITION_FAILED,
    T2032,
    "用户面容识别失败",
    "User Face Recognition Failed",
    "Specific case under user identity verification failed"
);

const_errtype!(
    ET_THIRD_PARTY_LOGIN_AUTHORIZATION_NOT_OBTAINED,
    T2033,
    "用户未获得第三方登录授权",
    "User Did Not Obtain Third-Party Login Authorization",
    "Specific case under user identity verification failed"
);

const_errtype!(
    ET_USER_LOGIN_EXPIRED,
    T2034,
    "用户登录已过期",
    "User Login Expired",
    "Specific case under user login error"
);

const_errtype!(
    ET_USER_VERIFICATION_CODE,
    T2035,
    "用户验证码错误",
    "User Verification Code Error",
    "Specific case under user login error"
);

const_errtype!(
    ET_USER_VERIFICATION_CODE_ATTEMPT_LIMIT_EXCEEDED,
    T2036,
    "用户验证码尝试次数超限",
    "User Exceeded Verification Code Attempt Limit",
    "Specific case under user verification code error"
);

const_errtype!(
    ET_ACCESS_AUTHORIZATION,
    T2037,
    "访问权限异常",
    "Access Authorization Error",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_ACCESS_UNAUTHORIZED,
    T2038,
    "访问未授权",
    "Access Unauthorized",
    "Specific case under access authorization error"
);

const_errtype!(
    ET_AUTHORIZATION_IN_PROGRESS,
    T2039,
    "正在授权中",
    "Authorization In Progress",
    "Specific case under access authorization error"
);

const_errtype!(
    ET_USER_AUTHORIZATION_APPLICATION_REJECTED,
    T2040,
    "用户授权申请被拒绝",
    "User Authorization Application Rejected",
    "Specific case under access authorization error"
);

const_errtype!(
    ET_ACCESS_BLOCKED_DUE_TO_PRIVACY_SETTINGS,
    T2041,
    "因访问对象隐私设置被拦截",
    "Access Blocked Due To Privacy Settings",
    "Specific case under access authorization error"
);

const_errtype!(
    ET_AUTHORIZATION_EXPIRED,
    T2042,
    "授权已过期",
    "Authorization Expired",
    "Specific case under access authorization error"
);

const_errtype!(
    ET_NO_PERMISSION_TO_USE_API,
    T2043,
    "无权限使用API",
    "No Permission To Use API",
    "Specific case under access authorization error"
);

const_errtype!(
    ET_USER_ACCESS_BLOCKED,
    T2044,
    "用户访问被拦截",
    "User Access Blocked",
    "Specific case under access authorization error"
);

const_errtype!(
    ET_BLACKLISTED_USER,
    T2045,
    "黑名单用户",
    "Blacklisted User",
    "Specific case under user access blocked"
);

const_errtype!(
    ET_ACCOUNT_FROZEN,
    T2046,
    "账号被冻结",
    "Account Frozen",
    "Specific case under user access blocked"
);

const_errtype!(
    ET_ILLEGAL_IP_ADDRESS,
    T2047,
    "非法IP地址",
    "Illegal IP Address",
    "Specific case under user access blocked"
);

const_errtype!(
    ET_GATEWAY_ACCESS_RESTRICTED,
    T2048,
    "网关访问受限",
    "Gateway Access Restricted",
    "Specific case under user access blocked"
);

const_errtype!(
    ET_REGIONAL_BLACKLIST,
    T2049,
    "地域黑名单",
    "Regional Blacklist",
    "Specific case under user access blocked"
);

const_errtype!(
    ET_SERVICE_OVERDUE,
    T2050,
    "服务已欠费",
    "Service Overdue",
    "Specific case under access authorization error"
);

const_errtype!(
    ET_USER_SIGNATURE,
    T2051,
    "用户签名异常",
    "User Signature Error",
    "Specific case under access authorization error"
);

const_errtype!(
    ET_RSA_SIGNATURE,
    T2052,
    "RSA签名错误",
    "RSA Signature Error",
    "Specific case under user signature error"
);

const_errtype!(
    ET_USER_REQUEST_PARAMETER,
    T2053,
    "用户请求参数错误",
    "User Request Parameter Error",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_ILLEGAL_MALICIOUS_LINK,
    T2054,
    "包含非法恶意跳转链接",
    "Contains Illegal Malicious Redirect Link",
    "Specific case under user request parameter error"
);

const_errtype!(
    ET_INVALID_USER_INPUT,
    T2055,
    "无效的用户输入",
    "Invalid User Input",
    "Specific case under user request parameter error"
);

const_errtype!(
    ET_REQUEST_PARAMETER_EMPTY,
    T2056,
    "请求必填参数为空",
    "Request Required Parameter Is Empty",
    "Specific case under user request parameter error"
);

const_errtype!(
    ET_USER_ORDER_NUMBER_EMPTY,
    T2057,
    "用户订单号为空",
    "User Order Number Is Empty",
    "Specific case under request required parameter is empty"
);

const_errtype!(
    ET_ORDER_QUANTITY_EMPTY,
    T2058,
    "订购数量为空",
    "Order Quantity Is Empty",
    "Specific case under request required parameter is empty"
);

const_errtype!(
    ET_MISSING_TIMESTAMP_PARAM,
    T2059,
    "缺少时间戳参数",
    "Missing Timestamp Parameter",
    "Specific case under request required parameter is empty"
);

const_errtype!(
    ET_ILLEGAL_TIMESTAMP_PARAM,
    T2060,
    "非法的时间戳参数",
    "Illegal Timestamp Parameter",
    "Specific case under request required parameter is empty"
);

const_errtype!(
    ET_PARAM_VALUE_EXCEEDS_RANGE,
    T2061,
    "请求参数值超出允许的范围",
    "Request Parameter Value Exceeds Allowed Range",
    "Specific case under user request parameter error"
);

const_errtype!(
    ET_PARAM_FORMAT_MISMATCH,
    T2062,
    "参数格式不匹配",
    "Parameter Format Mismatch",
    "Specific case under request parameter value exceeds allowed range"
);

const_errtype!(
    ET_ADDRESS_OUT_OF_SERVICE,
    T2063,
    "地址不在服务范围",
    "Address Out Of Service Range",
    "Specific case under request parameter value exceeds allowed range"
);

const_errtype!(
    ET_TIME_OUT_OF_SERVICE,
    T2064,
    "时间不在服务范围",
    "Time Out Of Service Range",
    "Specific case under request parameter value exceeds allowed range"
);

const_errtype!(
    ET_AMOUNT_EXCEEDS_LIMIT,
    T2065,
    "金额超出限制",
    "Amount Exceeds Limit",
    "Specific case under request parameter value exceeds allowed range"
);

const_errtype!(
    ET_QUANTITY_EXCEEDS_LIMIT,
    T2066,
    "数量超出限制",
    "Quantity Exceeds Limit",
    "Specific case under request parameter value exceeds allowed range"
);

const_errtype!(
    ET_BATCH_TOTAL_EXCEEDS_LIMIT,
    T2067,
    "请求批量处理总个数超出限制",
    "Request Batch Total Exceeds Limit",
    "Specific case under request parameter value exceeds allowed range"
);

const_errtype!(
    ET_JSON_PARSING_FAILED,
    T2068,
    "请求JSON解析失败",
    "Request JSON Parsing Failed",
    "Specific case under user request parameter error"
);

const_errtype!(
    ET_ILLEGAL_USER_INPUT,
    T2069,
    "用户输入内容非法",
    "User Input Content Illegal",
    "Specific case under user request parameter error"
);

const_errtype!(
    ET_PROHIBITED_SENSITIVE_WORDS,
    T2070,
    "包含违禁敏感词",
    "Contains Prohibited Sensitive Words",
    "Specific case under user input content illegal"
);

const_errtype!(
    ET_IMAGE_PROHIBITED_INFO,
    T2071,
    "图片包含违禁信息",
    "Image Contains Prohibited Information",
    "Specific case under user input content illegal"
);

const_errtype!(
    ET_FILE_INFRINGES_COPYRIGHT,
    T2072,
    "文件侵犯版权",
    "File Infringes Copyright",
    "Specific case under user input content illegal"
);

const_errtype!(
    ET_USER_OPERATION_ANOMALY,
    T2073,
    "用户操作异常",
    "User Operation Anomaly",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_USER_PAYMENT_TIMEOUT,
    T2074,
    "用户支付超时",
    "User Payment Timeout",
    "Specific case under user operation anomaly"
);

const_errtype!(
    ET_ORDER_CONFIRM_TIMEOUT,
    T2075,
    "确认订单超时",
    "Order Confirmation Timeout",
    "Specific case under user operation anomaly"
);

const_errtype!(
    ET_ORDER_CLOSED,
    T2076,
    "订单已关闭",
    "Order Closed",
    "Specific case under user operation anomaly"
);

const_errtype!(
    ET_USER_REQUEST_SERVICE_ANOMALY,
    T2077,
    "用户请求服务异常",
    "User Request Service Anomaly",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_REQUEST_FREQUENCY_EXCEEDED,
    T2078,
    "请求次数超出限制",
    "Request Frequency Exceeded",
    "Specific case under user request service anomaly"
);

const_errtype!(
    ET_REQUEST_CONCURRENCY_EXCEEDED,
    T2079,
    "请求并发数超出限制",
    "Request Concurrency Exceeded",
    "Specific case under user request service anomaly"
);

const_errtype!(
    ET_USER_OPERATION_WAIT,
    T2080,
    "用户操作请等待",
    "User Operation Please Wait",
    "Specific case under user request service anomaly"
);

const_errtype!(
    ET_WEBSOCKET_CONNECTION_ANOMALY,
    T2081,
    "WebSocket连接异常",
    "WebSocket Connection Anomaly",
    "Specific case under user request service anomaly"
);

const_errtype!(
    ET_WEBSOCKET_CONNECTION_DISCONNECTED,
    T2082,
    "WebSocket连接断开",
    "WebSocket Connection Disconnected",
    "Specific case under user request service anomaly"
);

const_errtype!(
    ET_USER_DUPLICATE_REQUEST,
    T2083,
    "用户重复请求",
    "User Duplicate Request",
    "Specific case under user request service anomaly"
);

const_errtype!(
    ET_USER_RESOURCE_ANOMALY,
    T2084,
    "用户资源异常",
    "User Resource Anomaly",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_INSUFFICIENT_ACCOUNT_BALANCE,
    T2085,
    "账户余额不足",
    "Insufficient Account Balance",
    "Specific case under user resource anomaly"
);

const_errtype!(
    ET_INSUFFICIENT_DISK_SPACE,
    T2086,
    "用户磁盘空间不足",
    "User Insufficient Disk Space",
    "Specific case under user resource anomaly"
);

const_errtype!(
    ET_INSUFFICIENT_MEMORY,
    T2087,
    "用户内存空间不足",
    "User Insufficient Memory Space",
    "Specific case under user resource anomaly"
);

const_errtype!(
    ET_OSS_CAPACITY_INSUFFICIENT,
    T2088,
    "用户OSS容量不足",
    "User OSS Capacity Insufficient",
    "Specific case under user resource anomaly"
);

const_errtype!(
    ET_QUOTA_EXHAUSTED,
    T2089,
    "用户配额已用光",
    "User Quota Exhausted",
    "Specific case under user resource anomaly"
);

const_errtype!(
    ET_FILE_UPLOAD_ANOMALY,
    T2090,
    "用户上传文件异常",
    "User File Upload Anomaly",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_FILE_TYPE_MISMATCH,
    T2091,
    "用户上传文件类型不匹配",
    "User File Upload Type Mismatch",
    "Specific case under user file upload anomaly"
);

const_errtype!(
    ET_FILE_TOO_LARGE,
    T2092,
    "用户上传文件太大",
    "User File Upload Too Large",
    "Specific case under user file upload anomaly"
);

const_errtype!(
    ET_IMAGE_TOO_LARGE,
    T2093,
    "用户上传图片太大",
    "User Image Upload Too Large",
    "Specific case under user file upload anomaly"
);

const_errtype!(
    ET_VIDEO_TOO_LARGE,
    T2094,
    "用户上传视频太大",
    "User Video Upload Too Large",
    "Specific case under user file upload anomaly"
);

const_errtype!(
    ET_COMPRESSED_FILE_TOO_LARGE,
    T2095,
    "用户上传压缩文件太大",
    "User Compressed File Upload Too Large",
    "Specific case under user file upload anomaly"
);

const_errtype!(
    ET_CURRENT_VERSION_ANOMALY,
    T2096,
    "用户当前版本异常",
    "User Current Version Anomaly",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_VERSION_INCOMPATIBLE,
    T2097,
    "用户安装版本与系统不匹配",
    "User Installed Version Incompatible With System",
    "Specific case under user current version anomaly"
);

const_errtype!(
    ET_VERSION_TOO_LOW,
    T2098,
    "用户安装版本过低",
    "User Installed Version Too Low",
    "Specific case under user current version anomaly"
);

const_errtype!(
    ET_VERSION_TOO_HIGH,
    T2099,
    "用户安装版本过高",
    "User Installed Version Too High",
    "Specific case under user current version anomaly"
);

const_errtype!(
    ET_VERSION_EXPIRED,
    T2100,
    "用户安装版本已过期",
    "User Installed Version Expired",
    "Specific case under user current version anomaly"
);

const_errtype!(
    ET_API_VERSION_MISMATCH,
    T2101,
    "用户API请求版本不匹配",
    "User API Request Version Mismatch",
    "Specific case under user current version anomaly"
);

const_errtype!(
    ET_API_VERSION_TOO_HIGH,
    T2102,
    "用户API请求版本过高",
    "User API Request Version Too High",
    "Specific case under user current version anomaly"
);

const_errtype!(
    ET_API_VERSION_TOO_LOW,
    T2103,
    "用户API请求版本过低",
    "User API Request Version Too Low",
    "Specific case under user current version anomaly"
);

const_errtype!(
    ET_PRIVACY_NOT_AUTHORIZED,
    T2104,
    "用户隐私未授权",
    "User Privacy Not Authorized",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_PRIVACY_NOT_SIGNED,
    T2105,
    "用户隐私未签署",
    "User Privacy Not Signed",
    "Specific case under user privacy not authorized"
);

const_errtype!(
    ET_CAMERA_DEVICE_NOT_AUTHORIZED,
    T2106,
    "用户摄像头未授权",
    "User Camera Device Not Authorized",
    "Specific case under user privacy not authorized"
);

const_errtype!(
    ET_CAMERA_NOT_AUTHORIZED,
    T2107,
    "用户相机未授权",
    "User Camera Not Authorized",
    "Specific case under user privacy not authorized"
);

const_errtype!(
    ET_PHOTO_LIBRARY_NOT_AUTHORIZED,
    T2108,
    "用户图片库未授权",
    "User Photo Library Not Authorized",
    "Specific case under user privacy not authorized"
);

const_errtype!(
    ET_FILE_ACCESS_NOT_AUTHORIZED,
    T2109,
    "用户文件未授权",
    "User Files Not Authorized",
    "Specific case under user privacy not authorized"
);

const_errtype!(
    ET_LOCATION_NOT_AUTHORIZED,
    T2110,
    "用户位置信息未授权",
    "User Location Information Not Authorized",
    "Specific case under user privacy not authorized"
);

const_errtype!(
    ET_CONTACT_LIST_NOT_AUTHORIZED,
    T2111,
    "用户通讯录未授权",
    "User Contact List Not Authorized",
    "Specific case under user privacy not authorized"
);

const_errtype!(
    ET_SYSTEM_ERROR,
    T3000,
    "系统执行出错",
    "System Execution Error",
    "Primary macroscopic error code"
);

const_errtype!(
    ET_SYSTEM_TIMEOUT,
    T3001,
    "系统执行超时",
    "System Execution Timeout",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_ORDER_PROCESSING_TIMEOUT,
    T3002,
    "系统订单处理超时",
    "System Order Processing Timeout",
    "Specific case under system execution timeout"
);

const_errtype!(
    ET_DISASTER_RECOVERY_TRIGGERED,
    T3003,
    "系统容灾功能被触发",
    "System Disaster Recovery Function Triggered",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_FLOW_RESTRICTION,
    T3004,
    "系统限流",
    "System Flow Restriction",
    "Specific case under system disaster recovery function triggered"
);

const_errtype!(
    ET_FUNCTIONALITY_DOWNGRADED,
    T3005,
    "系统功能降级",
    "System Functionality Downgraded",
    "Specific case under system disaster recovery function triggered"
);

const_errtype!(
    ET_SYSTEM_RESOURCE_ANOMALY,
    T3006,
    "系统资源异常",
    "System Resource Anomaly",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_RESOURCE_EXHAUSTED,
    T3007,
    "系统资源耗尽",
    "System Resources Exhausted",
    "Specific case under system resource anomaly"
);

const_errtype!(
    ET_DISK_SPACE_EXHAUSTED,
    T3008,
    "系统磁盘空间耗尽",
    "System Disk Space Exhausted",
    "Specific case under system resources exhausted"
);

const_errtype!(
    ET_MEMORY_EXHAUSTED,
    T3009,
    "系统内存耗尽",
    "System Memory Exhausted",
    "Specific case under system resources exhausted"
);

const_errtype!(
    ET_FILE_HANDLES_EXHAUSTED,
    T3010,
    "文件句柄耗尽",
    "File Handles Exhausted",
    "Specific case under system resources exhausted"
);

const_errtype!(
    ET_CONNECTION_POOL_EXHAUSTED,
    T3011,
    "系统连接池耗尽",
    "System Connection Pool Exhausted",
    "Specific case under system resources exhausted"
);

const_errtype!(
    ET_THREAD_POOL_EXHAUSTED,
    T3012,
    "系统线程池耗尽",
    "System Thread Pool Exhausted",
    "Specific case under system resources exhausted"
);

const_errtype!(
    ET_RESOURCE_ACCESS_ANOMALY,
    T3013,
    "系统资源访问异常",
    "System Resource Access Anomaly",
    "Specific case under system resource anomaly"
);

const_errtype!(
    ET_READ_DISK_FILE_FAILED,
    T3014,
    "系统读取磁盘文件失败",
    "System Failed To Read Disk File",
    "Specific case under system resource access anomaly"
);

const_errtype!(
    ET_ERROR_CALLING_THIRD_PARTY,
    T3015,
    "调用第三方服务出错",
    "Error Calling Third-Party Service",
    "Primary macroscopic error code"
);

const_errtype!(
    ET_MIDDLEWARE_SERVICE,
    T3016,
    "中间件服务出错",
    "Middleware Service Error",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_RPC_SERVICE,
    T3017,
    "RPC服务出错",
    "RPC Service Error",
    "Specific case under middleware service error"
);

const_errtype!(
    ET_RPC_SERVICE_NOT_FOUND,
    T3018,
    "RPC服务未找到",
    "RPC Service Not Found",
    "Specific case under RPC service error"
);

const_errtype!(
    ET_RPC_SERVICE_NOT_REGISTERED,
    T3019,
    "RPC服务未注册",
    "RPC Service Not Registered",
    "Specific case under RPC service error"
);

const_errtype!(
    ET_INTERFACE_NOT_EXIST,
    T3020,
    "接口不存在",
    "Interface Does Not Exist",
    "Specific case under RPC service error"
);

const_errtype!(
    ET_MESSAGE_SERVICE,
    T3021,
    "消息服务出错",
    "Message Service Error",
    "Specific case under middleware service error"
);

const_errtype!(
    ET_MESSAGE_DELIVERY,
    T3022,
    "消息投递出错",
    "Message Delivery Error",
    "Specific case under message service error"
);

const_errtype!(
    ET_MESSAGE_CONSUMPTION,
    T3023,
    "消息消费出错",
    "Message Consumption Error",
    "Specific case under message service error"
);

const_errtype!(
    ET_MESSAGE_SUBSCRIPTION,
    T3024,
    "消息订阅出错",
    "Message Subscription Error",
    "Specific case under message service error"
);

const_errtype!(
    ET_MESSAGE_GROUP_NOT_FOUND,
    T3025,
    "消息分组未查到",
    "Message Group Not Found",
    "Specific case under message service error"
);

const_errtype!(
    ET_CACHE_SERVICE,
    T3026,
    "缓存服务出错",
    "Cache Service Error",
    "Specific case under middleware service error"
);

const_errtype!(
    ET_KEY_LENGTH_EXCEEDS_LIMIT,
    T3027,
    "key长度超过限制",
    "Key Length Exceeds Limit",
    "Specific case under cache service error"
);

const_errtype!(
    ET_VALUE_LENGTH_EXCEEDS_LIMIT,
    T3028,
    "value长度超过限制",
    "Value Length Exceeds Limit",
    "Specific case under cache service error"
);

const_errtype!(
    ET_STORAGE_CAPACITY_FULL,
    T3029,
    "存储容量已满",
    "Storage Capacity Full",
    "Specific case under cache service error"
);

const_errtype!(
    ET_UNSUPPORTED_DATA_FORMAT,
    T3030,
    "不支持的数据格式",
    "Unsupported Data Format",
    "Specific case under cache service error"
);

const_errtype!(
    ET_CONFIGURATION_SERVICE,
    T3031,
    "配置服务出错",
    "Configuration Service Error",
    "Specific case under middleware service error"
);

const_errtype!(
    ET_NETWORK_RESOURCE_SERVICE,
    T3032,
    "网络资源服务出错",
    "Network Resource Service Error",
    "Specific case under middleware service error"
);

const_errtype!(
    ET_VPN_SERVICE,
    T3033,
    "VPN服务出错",
    "VPN Service Error",
    "Specific case under network resource service error"
);

const_errtype!(
    ET_CDN_SERVICE,
    T3034,
    "CDN服务出错",
    "CDN Service Error",
    "Specific case under network resource service error"
);

const_errtype!(
    ET_DOMAIN_NAME_RESOLUTION_SERVICE,
    T3035,
    "域名解析服务出错",
    "Domain Name Resolution Service Error",
    "Specific case under network resource service error"
);

const_errtype!(
    ET_GATEWAY_SERVICE,
    T3036,
    "网关服务出错",
    "Gateway Service Error",
    "Specific case under network resource service error"
);

const_errtype!(
    ET_THIRD_PARTY_TIMEOUT,
    T3037,
    "第三方系统执行超时",
    "Third-Party System Execution Timeout",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_RPC_TIMEOUT,
    T3038,
    "RPC执行超时",
    "RPC Execution Timeout",
    "Specific case under third-party system execution timeout"
);

const_errtype!(
    ET_MESSAGE_DELIVERY_TIMEOUT,
    T3039,
    "消息投递超时",
    "Message Delivery Timeout",
    "Specific case under third-party system execution timeout"
);

const_errtype!(
    ET_CACHE_TIMEOUT,
    T3040,
    "缓存服务超时",
    "Cache Service Timeout",
    "Specific case under third-party system execution timeout"
);

const_errtype!(
    ET_CONFIGURATION_TIMEOUT,
    T3041,
    "配置服务超时",
    "Configuration Service Timeout",
    "Specific case under third-party system execution timeout"
);

const_errtype!(
    ET_DATABASE_TIMEOUT,
    T3042,
    "数据库服务超时",
    "Database Service Timeout",
    "Specific case under third-party system execution timeout"
);

const_errtype!(
    ET_DATABASE_SERVICE,
    T3043,
    "数据库服务出错",
    "Database Service Error",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_TABLE_NOT_EXIST,
    T3044,
    "表不存在",
    "Table Does Not Exist",
    "Specific case under database service error"
);

const_errtype!(
    ET_COLUMN_NOT_EXIST,
    T3045,
    "列不存在",
    "Column Does Not Exist",
    "Specific case under database service error"
);

const_errtype!(
    ET_ROW_NOT_EXIST,
    T3046,
    "行不存在",
    "Row Does Not Exist",
    "Specific case under database service error"
);

const_errtype!(
    ET_MULTIPLE_IDENTICAL_COLUMN_NAMES,
    T3047,
    "多表关联中存在多个相同名称的列",
    "Multiple Identical Column Names In Multi-Table Association",
    "Specific case under database service error"
);

const_errtype!(
    ET_DATABASE_DEADLOCK,
    T3048,
    "数据库死锁",
    "Database Deadlock",
    "Specific case under database service error"
);

const_errtype!(
    ET_PRIMARY_KEY_CONFLICT,
    T3049,
    "主键冲突",
    "Primary Key Conflict",
    "Specific case under database service error"
);

const_errtype!(
    ET_THIRD_PARTY_DISASTER_RECOVERY_TRIGGERED,
    T3050,
    "第三方容灾系统被触发",
    "Third-Party Disaster Recovery System Triggered",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_THIRD_PARTY_FLOW_RESTRICTED,
    T3051,
    "第三方系统限流",
    "Third-party System Flow Restriction",
    "Specific case under third-party disaster recovery system triggered"
);

const_errtype!(
    ET_THIRD_PARTY_FUNCTIONALITY_DOWNGRADED,
    T3052,
    "第三方功能降级",
    "Third-party Functionality Downgrade",
    "Specific case under third-party disaster recovery system triggered"
);

const_errtype!(
    ET_NOTIFICATION_SERVICE,
    T3053,
    "通知服务出错",
    "Notification Service Error",
    "Secondary macroscopic error code"
);

const_errtype!(
    ET_SMS_REMINDER_FAILURE,
    T3054,
    "短信提醒服务失败",
    "SMS Reminder Service Failure",
    "Specific case under notification service error"
);

const_errtype!(
    ET_VOICE_REMINDER_FAILURE,
    T3055,
    "语音提醒服务失败",
    "Voice Reminder Service Failure",
    "Specific case under notification service error"
);

const_errtype!(
    ET_EMAIL_REMINDER_FAILURE,
    T3056,
    "邮件提醒服务失败",
    "Email Reminder Service Failure",
    "Specific case under notification service error"
);
