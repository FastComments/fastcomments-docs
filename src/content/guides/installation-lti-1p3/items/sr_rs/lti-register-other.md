#### Sakai

Sakai подржава LTI 1.3 Dynamic Registration на издањима са LTI Advantage. Из Administration Workspace-а:

1. Пријавите се као Sakai администратор и отворите **Administration Workspace**.
2. Изаберите **External Tools** > **Install LTI 1.3 Tool**.
3. Налепите FastComments URL за регистрацију (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">преузмите га овде</a>) и потврдите.
4. Одобрите алат када се руковање заврши.

Алат се затим појављује под **External Tools** и може га додати на сајтове њихови одржаваоци.

#### Schoology

Schoology Enterprise инстанце подржавају LTI 1.3, али доступност Dynamic Registration-а варира у зависности од примене. Проверите са својим Schoology менаџером налога.

Ако Dynamic Registration није доступан на вашој Schoology инстанци, мораћете ручно да конфигуришете интеграцију користећи ове крајње тачке:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Након што Schoology вам да Client ID и Deployment ID, контактирајте FastComments подршку да региструје конфигурацију на вашем tenant-у.

#### Остале LTI 1.3 платформе

Било који LMS који прати IMS LTI 1.3 Advantage спецификацију требало би да ради са истим URL-ом за регистрацију (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">преузмите га овде</a>). Потражите подешавање означено као "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" или слично.

Ако ваша платформа подржава само ручно LTI 1.3 подешавање, користите четири крајње тачке наведене у Schoology одељку изнад и контактирајте подршку да завршите.