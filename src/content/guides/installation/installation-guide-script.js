const path = require('path');
const fs = require('fs');
const {GUIDE_ORDER_PATH} = require('../../../guides');
const {getGuideIndex} = require('../../../guides');
const {TEMPLATE_DIR} = require('../../../guides');
const {ITEMS_DIR_NAME} = require('../../../guides');
const {GUIDES_DIR} = require('../../../guides');
const {createPageLink} = require('../../../guides');

function createId(text) {
    return text.replace(/ /g, '-').toLowerCase();
}

module.exports = function getGuidePages(meta) {
    const pages = [];

    const guideId = createId(meta.name);

    const guideOrder = JSON.parse(fs.readFileSync(GUIDE_ORDER_PATH, 'utf8'));
    const guideIndex = getGuideIndex(guideId, guideOrder);

    meta.productsOrdered.forEach(function (item) {
        item.url = createPageLink(guideId + '-' + createId(item.name));
    });

    // build product select page
    pages.push({
        id: guideId,
        url: createPageLink(guideId),
        icon: `images/guide-icons/${meta.icon}`,
        name: meta.name,
        itemsPath: path.join(GUIDES_DIR, guideId, ITEMS_DIR_NAME),
        pageLayoutPath: path.join(TEMPLATE_DIR, 'layout-installation-pick-product.html'),
        data: {
            items: meta.productsOrdered
        }
    });

    // build integration select pages, and integration + product pages
    for (const productMeta of meta.productsOrdered) {
        const productId = createId(productMeta.name);
        const integrationsMetaForProduct = meta.integrationsOrdered
            .filter((integrationMeta) => {
                return !integrationMeta.supportedProducts || integrationMeta.supportedProducts.includes(productId);
            })
            .map((integrationMeta) => {
            return {
                ...integrationMeta,
                url: createPageLink(`${guideId}-${productId}-${createId(integrationMeta.name)}`)
            }
        });
        const integrationSelectPage = {
            id: guideId,
            url: productMeta.url,
            name: meta.name,
            itemsPath: path.join(GUIDES_DIR, guideId, ITEMS_DIR_NAME),
            pageLayoutPath: path.join(TEMPLATE_DIR, 'layout-installation-pick-integration.html'),
            data: {
                items: integrationsMetaForProduct
            }
        };
        pages.push(integrationSelectPage);
        for (const integrationMeta of integrationsMetaForProduct) {
            const integrationProductPage = {
                id: guideId,
                prevGuideUrl: guideIndex > 0 ? createPageLink(guideOrder[guideIndex - 1]) : null,
                nextGuideUrl: guideIndex > -1 && guideIndex < guideOrder.length - 1 ? createPageLink(guideOrder[guideIndex + 1]) : null,
                url: integrationMeta.url,
                name: meta.name,
                itemsPath: path.join(GUIDES_DIR, guideId, ITEMS_DIR_NAME),
                pageLayoutPath: path.join(TEMPLATE_DIR, 'layout-installation-via-code.html'),
                data: {

                }
            };
            pages.push(integrationProductPage);
        }
    }

    return pages;
}

// TODO able to call this
async function createProductInstructionsHTML() {
    // TODO generate product instructions

}
