---
Визначення того, як кілька користувачів взаємодіють, і тестування цього є складним. Ось специфікація, яку ми використовуємо для контролю доступу,
яку ви можете використати для тестування вашої реалізації:

    Page with null group ids, user with null group ids - повинен мати доступ.
    Page with null group ids, user with group ids - повинен мати доступ.
    Page with group ids, user with null group ids - повинен мати доступ.
    Page with group ids, user with empty list - НЕ повинен мати доступ.
    Page with group ids, user with group ids - intersection exists - повинен мати доступ.
    Page with group ids, user with group ids - intersection does not exist - НЕ повинен мати доступ.
    Page with empty list of group ids (nobody has access), user with null - НЕ повинен мати доступ.
    
    SSO User A = group ids не визначені (null = повний доступ).
    SSO User B = group ids не визначені (null = повний доступ).
    A може @B.
    
    SSO User A = group ids не визначені (null = повний доступ).
    SSO User B = group ids визначені.
    A може @B.
    
    SSO User A = group ids визначені.
    SSO User B = group ids не визначені (null = повний доступ).
    A може @B.
    
    SSO User A = group ids = [a].
    SSO User B = group ids = [b].
    A НЕ може @B.
    
    SSO User A = group ids = [a].
    SSO User B = group ids = [a, b].
    A може @B.

---