## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | number | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getTenantPackages उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5ad8b1e4-8c1b-4f2a-9d3e-7f6a1b2c3d4e';

// टेनेंट पैकेजों का पहला पेज प्राप्त करें
const response: GetTenantPackagesResponse = await getTenantPackages(tenantId);
const firstPackage: TenantPackage | undefined = response.packages?.[0];
if (firstPackage?.id) {
  console.log(`First package: ${firstPackage.id} - active: ${firstPackage.active}`);
}

// skip का उपयोग करके अगला पेज प्राप्त करें
const skip: number = 10;
const nextPage: GetTenantPackagesResponse = await getTenantPackages(tenantId, skip);
const status: APIStatus = nextPage.status;
console.log(`Fetched ${nextPage.packages?.length ?? 0} packages, status: ${status}`);
[inline-code-end]