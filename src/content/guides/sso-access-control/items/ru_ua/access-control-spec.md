---
Визначення того, як кілька користувачів взаємодіють, і тестування цього — складне завдання. Ось специфікація, якої ми дотримуємося для контролю доступу, яку ви можете використовувати для тестування вашої реалізації:

    Сторінка with null group ids, користувач with null group ids - має мати доступ.
    Сторінка with null group ids, користувач with group ids - має мати доступ.
    Сторінка with group ids, користувач with null group ids - має мати доступ.
    Сторінка with group ids, користувач with empty list - НЕ має мати доступ.
    Сторінка with group ids, користувач with group ids - перетин існує - має мати доступ.
    Сторінка with group ids, користувач with group ids - перетин не існує - НЕ має мати доступ.
    Сторінка with empty list of group ids (ніхто не має доступу), користувач with null - НЕ має мати доступ.
    
    SSO User A = group ids не визначені (null = повний доступ).
    SSO User B = group ids не визначені (null = повний доступ).
    A може @B.
    
    SSO User A = group ids не визначені (null = повний доступ).
    SSO User B = group ids визначені.
    A може @B.
    
    SSO User A = group ids визначені.
    SSO User B = group ids не визначені (null = повний доступ).
    A може @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    A НЕ може @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A може @B.

---