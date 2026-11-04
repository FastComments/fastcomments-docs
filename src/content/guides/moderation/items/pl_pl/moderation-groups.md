Moderatorzy mogą być przypisywani do grup w celu moderowania różnych stron lub kategorii treści.

Gdy Moderator należy do jednej lub więcej grup, będzie widział komentarze tylko z tych grup na stronie Moderate Comments.

Na przykład, załóżmy, że prowadzimy stronę wyświetlającą filmy wideo według kategorii. Możemy chcieć mieć różnych moderatorów dla filmów o kotach, psach i papugach, więc [dodajmy te grupy](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Lista grup moderacyjnych z utworzonymi grupami Cat, Dog i Parrot dla każdej kategorii filmów'; title='Strona grup moderacyjnych' app-screenshot-end]

Gdy dodajemy moderatora, mamy teraz możliwość wybrania jednej lub więcej grup, do których moderator będzie należał:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Formularz dodawania moderatora z selektorem grup służącym do przypisania moderatora do jednej lub więcej grup'; title='Dodawanie moderatora i wybieranie grupy' app-screenshot-end]

Na koniec, komentarze muszą być powiązane z jedną lub więcej grupami, aby widzieli je właściwi moderatorzy.

Można to skonfigurować poprzez [dodanie grup](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups), a następnie określenie odpowiednich identyfikatorów `Moderation Group` w widżecie komentarzy,
[ zgodnie z instrukcjami tutaj](/guide-customizations-and-configuration.html#moderation-group-ids).