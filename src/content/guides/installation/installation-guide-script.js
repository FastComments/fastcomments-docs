module.exports = function getGuidePages(meta) {
    const pages = [];

    // build product select page

    // build integration select pages, and integration + product pages
    for (const productMeta of meta.productsOrdered) {
        for (const integrationMeta of meta.integrationsOrdered) {
            const integrationSelectPage = null;
            pages.push(integrationSelectPage);
            const integrationProductPage = null;
            pages.push(integrationProductPage);
        }
    }

    return pages;
}
