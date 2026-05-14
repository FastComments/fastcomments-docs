Blackboard Learn SaaS и Ultra подржавају LTI 1.3 динамичку регистрацију.

#### Отворите екран добављача алата

1. Пријавите се у Blackboard као системски администратор.
2. Навигирајте до **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Кликните **Register LTI 1.3 / LTI Advantage Tool**.

Ако видите само "Register LTI 1.1 Provider", ваша верзија Blackboard-а још не подржава LTI 1.3 - надоградите или контактирајте Blackboard подршку.

#### Залепите URL

Залепите FastComments регистрациони URL у поље **Client ID** / **Registration URL** (ознака у Blackboard-у варира по верзији). Пошаљите.

Blackboard обавља процес регистрације (handshake) са FastComments и приказује екран са потврдом.

#### Одобрите и омогућите

Blackboard подразумевано означава новорегистроване алате као **Approved but excluded**:

1. Пронађите унос FastComments у листи добављача алата.
2. Отворите мени и одаберите **Edit**.
3. Поставите **Tool Status** на **Approved**.
4. Под **Institution Policies**, прегледајте које корисничке податке се шаљу (име, е-пошта, улога). Сачувајте.

Алат је сада доступан предавачима када додају садржај у курсеве.