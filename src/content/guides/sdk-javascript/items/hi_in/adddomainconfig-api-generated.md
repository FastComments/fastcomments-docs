## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| addDomainConfigParams | AddDomainConfigParams | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddDomainConfigResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'addDomainConfig उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const config: AddDomainConfigParams = {
    domain: 'myblog.example.com',
    enabled: true,
    // description वैकल्पिक है और यहाँ छोड़ा गया है
  };
  const response: AddDomainConfigResponse = await addDomainConfig(tenantId, config);
  console.log(response);
})();
[inline-code-end]