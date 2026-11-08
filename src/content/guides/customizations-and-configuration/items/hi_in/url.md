[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

नोटिफिकेशन ईमेल भेजते समय, या मॉडरेशन पेज जैसे यूज़र इंटरफेस में कमेंट्स रेंडर करते समय, कमेंट से उसके पेज तक लिंक बनाने में सक्षम होना उपयोगी होता है।

यदि URL ID हमेशा एक ID नहीं है, तो हमें URL को किसी अन्य स्थान पर स्टोर करना होगा। इसी काम के लिए "url" प्रॉपर्टी है, जिसे निम्न प्रकार परिभाषित किया गया है।

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'कस्टम URL परिभाषित करना'; code-example-end]

एक सामान्य उपयोग का मामला कमेंट थ्रेड को किसी पहचानकर्ता, जैसे एक आर्टिकल, से जोड़ना और फिर किसी विशेष पेज पर वापस लिंक करना है, उदाहरण के लिए:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'कस्टम URL और URL IDs को एक साथ परिभाषित करना'; code-example-end]

URL से सामान्य मार्केटिंग पैरामीटर नहीं हटाए जाते। डिफ़ॉल्ट रूप से, जो भी वर्तमान पेज URL है, वही URL कमेंट के साथ स्टोर किया जाता है।