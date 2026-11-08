यह Drupal निर्देशों का "too long did not read" संस्करण है।

1. मॉड्यूल को `composer require drupal/fcom` से इंस्टॉल करें, या इसे `modules/custom/fastcomments/` में डाल दें।
2. इसे `drush en fastcomments` से सक्षम करें, या `/admin/modules` पर एडमिन UI से करें।
3. `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`) पर जाएँ।
4. [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)) से अपनी Tenant ID और API Secret दर्ज करें।
5. `Structure > Content types > [type] > Manage fields` के माध्यम से किसी भी कंटेंट टाइप में `FastComments` फ़ील्ड जोड़ें।

मॉड्यूल [drupal.org/project/fcom](https://www.drupal.org/project/fcom) पर प्रकाशित है।