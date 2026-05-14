#### Sakai

Sakai подржава LTI 1.3 динамичку регистрацију на верзијама са LTI Advantage. Из Административног радног простора:

1. Пријавите се као администратор Sakai-а и отворите **Административни радни простор**.
2. Изаберите **Спољни алати (External Tools)** > **Инсталирај LTI 1.3 алатку (Install LTI 1.3 Tool)**.
3. Налепите FastComments URL за регистрацију (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">добите га овде</a>) и пошаљите.
4. Одобрите алат када руковање заврши.

Алат се затим појављује под **Спољни алати (External Tools)** и може га додати одржавач сајта.

#### Schoology

Schoology Enterprise инстанце подржавају LTI 1.3, али доступност динамичке регистрације варира у зависности од распоређивања. Проверите са вашим Schoology менаџером налога.

Ако динамичка регистрација није доступна на вашој Schoology инстанци, мораћете да конфигуришете интеграцију ручно користећи следеће крајње тачке:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Након што вам Schoology додели Client ID и Deployment ID, контактирајте FastComments подршку да бисте регистровали конфигурацију на вашем tenant-у.

#### Other LTI 1.3 Platforms

Било који LMS који прати IMS LTI 1.3 Advantage спецификацију требало би да ради са истим URL-ом за регистрацију (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">добите га овде</a>). Потражите подешавање означено као „Динамичка регистрација“, „URL за регистрацију алата (Tool Registration URL)“, „крајња тачка за иницирање регистрације алата (Tool initiation registration endpoint)“ или слично.

Ако ваша платформа подржава само ручно подешавање LTI 1.3, користите четири крајње тачке наведене у одељку Schoology изнад и контактирајте подршку да бисте финализовали.