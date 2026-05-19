FastComments LTI 1.3 интеграција се придржава принципа најмањих привилегија: користи само launch claim-ове потребне да идентификује корисника, повежe коментаре са правилним курсом и ресурсом, и примене дозволе базиране на улогама.

Остатак ове странице мапира сваки claim који интеграција користи, све LTI Advantage сервисе које не захтијева, и све категорије података које не прикупља. Рецензенти за сигурност и набавку могу директно преузети одговоре из табела у наставку.

## Елементи података примљени од LMS-а

Сваки LTI 1.3 launch носи потписани JWT од LMS-а. FastComments издваја сљедеће claim-ове из тог JWT-а и не користи ништа друго:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | Идентификује корисника конзистентно преко launch-ова тако да иста особа одговара истом FastComments SSO кориснику | Yes | Yes, as part of a stable internal SSO ID |
| Display name | `name` | Ауторство приказано поред коментара корисника | Yes (falls back to "LMS User" if absent) | Yes |
| Email | `email` | Упаривање налога, обавијести, модерација, кореспонденција за подршку | Optional (the integration works without it) | Yes when provided |
| Avatar URL | `picture` | Приказано на коментарима корисника | Optional | URL only; FastComments does not download or rehost the image |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Одређује да ли је корисник администратор, предавач (модератор) или студент | Yes | Derived `isAdmin` / `isModerator` flags on the SSO session |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Повеzuje thread коментара са исправним LMS курсом | Yes | Yes, as part of the resolved page identifier |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Повеzuje коментаре са исправном активношћу или позицијом алата унутар курса | Yes when present | Yes, as part of the resolved page identifier |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Усмјерава launch на правилну FastComments tenant конфигурацију | Yes | Yes, on the FastComments LTI configuration record |

## Claims и scope-ови декларисани при регистрацији

Током LTI 1.3 Dynamic Registration, FastComments се региструје са `scope: ""` (нема додатних OAuth опсега) и декларише само ове OpenID Connect claim-ове:

`iss`, `sub`, `name`, `email`, `picture`

Региструје два типа порука:

- `LtiResourceLinkRequest` - стандардно покретање курса у FastComments.
- `LtiDeepLinkingRequest` - омогућава предавачима да поставе FastComments алат унутар курса.

Никакви додатни access token-ови се не захтијевају од LMS-а.

## LTI Advantage сервиси који се НЕ ЗАХТЈЕВАЈУ

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | No | Интеграцији није потребна листа курса; идентитет корисника стиже са сваким launch-ом |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | No | Интеграција није повезана са оценама |
| Deep Linking beyond the standard placement return | No additional data | Deep linking се користи само за постављање алата од стране предавача; ништа од курса се не инвентарише |

## Податоци који се НЕ ПРИКУПЉАЈУ

Поред самог LTI-а, FastComments не захтијева и не прима сљедеће од LMS-а или корисника:

| Category | Collected? |
|----------|------------|
| Student grades | No |
| Assignment submissions | No |
| Attendance records | No |
| Full course rosters | No |
| Government identifiers | No |
| Date of birth | No |
| Postal address or phone number | No |
| Financial information | No |
| LMS administrator credentials | No |

## Границе приступа

- FastComments прима податке само у оквиру овлашћеног LTI 1.3 launch-а потписаног кључевима регистрованим код LMS-а. Интеграција не позива LMS за додатне информације.
- Launch токени су за једнократну употребу и кратког вијека трајања. Поновљени или истекли токени се одбијају.
- LMS администратори контролишу гдје је алат распоређен унутар њихове платформе. D2L Brightspace, на примјер, подржава опсег по deployment-у за org-unit и безбједносне поставке по deployment-у, што омогућава администраторима да ограниче алат на одређене курсеве или организационе јединице уместо да га учине доступним глобално. Moodle, Blackboard, Sakai и Schoology нуде еквивалентне контроле по deployment-у у својим LTI 1.3 имплементацијама.

## Чување и ретенција

FastComments чува LTI-изведене податке током трајања активне услуге коментарисања и у складу са подешавањима задржавања које конфигурише купац. Подаци о коментарима се чувају у шифрованом складишту у продукцији. При отказу налога или писменом захтјеву за брисање, FastComments брише или анонимизује податке купца у складу са примјењивим уговором.

За потпунa појашњења о складиштењу и руковању подацима, видите <a href="https://fastcomments.com/privacy-policy" target="_blank">Политику приватности FastComments</a>.

## Ревизија и преглед

Свака нова LTI функција која би захтијевала додатне claim-ове, scope-ове или LTI Advantage сервисе прегледа се прије објаве како би се потврдило да је затражени приступ неопходан и пропорционалан реализованој функцији.

## Кратка изјава за упитнике о безбједности

> FastComments примјењује принцип најмањих привилегија и минимизацију података у својој LTI 1.3 интеграцији. Интеграција користи само LTI launch claim-ове потребне за аутентификацију корисника (`sub`, `name`, `email`, `picture`), одређивање њихове улоге и идентификацију курса и ресурса на које коментари припадају. FastComments не захтијева Names and Role Provisioning Services, Assignment and Grade Services, податке из дневника оцјена, присуство, пуне листе учесника или административни приступ LMS-у. LMS администратори задржавају контролу над тим у којим организационим јединицама, курсевима и deployment-има је алат доступан.