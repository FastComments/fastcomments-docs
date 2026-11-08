जेनरेटर, लोकल रूप से चल रहे FastComments सर्वर से स्पेक ले लेता है
(`http://localhost:3001/js/swagger.json`) जब उपलब्ध हो, अन्यथा कमिट किए गए
`openapi.json` पर फॉलबैक करता है।

```bash
python3 update.py
```

इसके लिए `node`/`npx` (`@openapitools/openapi-generator-cli` के लिए) और Java आवश्यक है।