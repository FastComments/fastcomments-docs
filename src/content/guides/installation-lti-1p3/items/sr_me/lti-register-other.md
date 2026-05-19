#### Sakai

Sakai подржава LTI 1.3 Dynamic Registration на верзијама са LTI Advantage. Из Administration Workspace:

1. Пријавите се као Sakai администратор и отворите **Administration Workspace**.
2. Изаберите **External Tools** > **Install LTI 1.3 Tool**.
3. Налепите FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">преузмите га овде</a>) и пошаљите.
4. Одобрите алат када руковање буде завршено.

Алат ће се затим појавити под **External Tools** и може га додати одржавач сајта.

#### Schoology

Schoology Enterprise инстанце подржавају LTI 1.3, али доступност "Dynamic Registration" варира у зависности од окружења. Проверите са вашим Schoology менаџером налога.

Ако "Dynamic Registration" није доступна на вашој Schoology инстанци, мораћете да конфигуришете интеграцију ручно користећи ове ендпоинте:

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Након што вам Schoology обезбеди Client ID и Deployment ID, контактирајте FastComments подршку да региструјете конфигурацију на вашем tenant-у.

#### Other LTI 1.3 Platforms

Било који LMS који прати IMS LTI 1.3 Advantage спецификацију треба да ради са истим registration URL-ом (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">преузмите га овде</a>). Потражите поставку означену као "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint" или слично.

Ако ваша платформа подржава само ручно подешавање LTI 1.3, користите четири ендпоинта наведена у одељку за Schoology изнад и контактирајте подршку да окончате подешавање.