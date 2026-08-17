---
По подразбиране FastComments ще показва името на потребителя така, както е въведено, или както е предадено чрез SSO.

Въпреки това, може да е желателно да се маскира или покаже името на потребителя по различен начин. Например, ако името на потребителя е Allen Rex, може би искате да показвате само „Allen R.“.

Това може да се направи без код в потребителския интерфейс за персонализиране на уиджета, под настройката, наречена `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Падащо меню Commenter Name Format отворено с опции като Capitalize, Last Initial и All Initials'; title='Промяна на формата на името' app-screenshot-end]

Наличните формати са:

- Capitalize (показва примерен потребител като Example User)
- Last Initial (показва Example User като Example U.)
- All Initials (показва Example User като E. U.)
- Show "Anonymous"

Ефектът от промяната е незабавен. Потребителите все още ще виждат пълното си потребителско име в горната част на областта за коментари, за себе си, но техните коментари ще показват модифицираното потребителско име.

Потребителските имена се маскират от сървъра, за защита на потребителите.
---