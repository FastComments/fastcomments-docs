प्लगइन के काम करने के लिए, एक टोकन आपके WordPress डेटाबेस में और साथ ही आपके FastComments खाते में सहेजा जाता है। जब प्लगइन हमारे सर्वरों को अनुरोध भेजता है, तो यह यह टोकन प्रदान करता है।

आप अपने FastComments खाते को अधिकृत सभी इंटीग्रेशन [यहाँ](https://fastcomments.com/auth/my-account/manage-data/integrations) देख सकते हैं।

सारी संचार HTTPS के माध्यम से किया जाता है।

सारी संचार आपके WordPress सर्वर से FastComments.com *की ओर* *आउटबाउंड* होती है, जिसमें आपके WordPress इंस्टॉलेशन पर वापस सिंक शामिल है, क्योंकि इसे आपके WordPress इंस्टॉलेशन में [cron](https://en.wikipedia.org/wiki/Polling_(computer_science) सेटअप से [पोलिंग](https://developer.wordpress.org/plugins/cron/)) के माध्यम से लागू किया गया है।