#### Navigate to LTI 1.3 Configuration

Пријавите се у FastComments и идите на <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">вашу LTI 1.3 страницу конфигурације</a>.

Ако ваш налог још нема приступ LTI-ју, видећете "LTI not enabled for this account" - обратите се подршци да га омогуће на вашем плану.

#### Pick a Platform (Optional)

Под **Generate a Dynamic Registration URL**, користите падајући мени **Platform** да кажете FastComments којем LMS-у се повезујете:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Друга LTI 1.3 платформа

Такође можете оставити на **Auto-detect**. Платформа се чита из openid-configuration вашег LMS-а током регистрације; падајући мени само поставља етикету за приказ за добијену конфигурацију.

#### Generate the URL

Кликните на **Generate URL**. FastComments креира једнократни registration token и прикаже вам URL који изгледа овако:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Копирајте га. Овај URL:

- Је **за једнократну употребу** - када ваш LMS успешно позове URL, токен се троши.
- Истиче након **30 минута** ако се не употреби.
- Треба да остане приватан - било ко са URL-ом може регистровати алат за вашег tenant-а у року тих 30 минута.

#### Existing Configurations

Када се регистрација успешно заврши, нова конфигурација ће се појавити у табели **Existing Configurations** на истој страници, са својом Platform, Issuer, Client ID, и Status. Из ове табеле можете избрисати конфигурације ако икада требате да поништите регистрацију.