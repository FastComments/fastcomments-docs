FastComments Drupal मॉड्यूल Drupal के बिल्ट-इन कमेंट्स को एक तेज़, रियल-टाइम कमेंटिंग सिस्टम से बदल देता है। यह मॉड्यूल [drupal.org पर प्रकाशित है](https://www.drupal.org/project/fcom) और Drupal 10 तथा 11 के साथ काम करता है।

इसे इंस्टॉल करने के दो तरीके हैं।

## Composer के साथ इंस्टॉल करें

```
composer require drupal/fcom
drush en fastcomments
```

## मैन्युअल रूप से इंस्टॉल करें

मॉड्यूल को [drupal.org/project/fcom](https://www.drupal.org/project/fcom) से डाउनलोड करें और इसे अपनी साइट की `modules/custom/fastcomments/` डायरेक्टरी में रखें। फिर इसे `drush en fastcomments` से, या एडमिन UI में `Extend` (`/admin/modules`) से सक्रिय करें।

<sup>ध्यान दें!</sup> यह मॉड्यूल केवल Drupal core (`user` और `field`) पर निर्भर करता है। किसी अन्य Drupal मॉड्यूल या लाइब्रेरी की आवश्यकता नहीं है।

मॉड्यूल सक्रिय होने के बाद, अपनी Tenant ID और API Secret सेट अप करने के लिए `Configuration` सेक्शन पर जाएँ।