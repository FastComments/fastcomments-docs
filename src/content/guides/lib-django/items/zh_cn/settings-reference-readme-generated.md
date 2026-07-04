| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | 您的 FastComments 租户 ID（测试用为 `demo`）。 |
| `API_KEY` | `""` | 您的 API 密钥。用于签署安全 SSO 并验证 `admin()`。 |
| `REGION` | `None` | 美国使用 `None`，欧盟地区使用 `"eu"`。 |
| `SSO.ENABLED` | `False` | 启用 SSO。 |
| `SSO.MODE` | `"secure"` | `"secure"`（HMAC）或 `"simple"`（未签名）。 |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | 显示给未登录的访客；默认使用 `reverse("login"/"logout")`。 |
| `SSO.USER_MAP` | id/email/username | FastComments 字段映射到用户属性/路径/可调用对象。 |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` 或点分路径。 |
| `SSO.USER_MAPPER` | `None` | 点分路径指向 `callable(user) -> dict`；优先级最高。 |
| `WIDGET_DEFAULTS` | `{}` | 配置将合并到每个小部件中（camelCase 键）。 |