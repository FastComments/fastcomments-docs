1. Увійдіть у FastComments і перейдіть на <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">your LTI 1.3 Configuration page</a>.
2. (Необов'язково) Виберіть платформу, з якої ви підключаєтеся, у спадному списку **Platform** - це задає мітку відображення, але Auto-detect працює добре.
3. Клацніть **Generate URL**. З'явиться одноразовий **Registration URL** (дійсний 30 хвилин, для одноразового використання).
4. У вашому LMS відкрийте екран LTI 1.3 Dynamic Registration і вставте URL у поле **Tool initiation registration endpoint** (або еквівалентне). Надішліть.
5. Ваш LMS викликає FastComments у відповідь, обмінюється ключами і створює інтеграцію. Коли операція завершена, спливаюче вікно закривається автоматично.
6. Повернувшись у FastComments, нова конфігурація з'явиться в таблиці **Existing Configurations**. Інструмент тепер доступний у курсах вашого LMS.