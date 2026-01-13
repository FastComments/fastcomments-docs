const fs = require('fs');
const path = require('path');

const GUIDES_DIR = path.join(__dirname, 'content/guides');

function findEmptyCodeFiles() {
    const emptyFiles = [];

    // Find all SDK guide directories
    const sdkDirs = fs.readdirSync(GUIDES_DIR).filter(d => d.startsWith('sdk-'));

    for (const sdkDir of sdkDirs) {
        const generatedDir = path.join(GUIDES_DIR, sdkDir, 'items', 'generated');

        if (!fs.existsSync(generatedDir)) {
            continue;
        }

        // Only check API files - README files don't have code blocks
        const files = fs.readdirSync(generatedDir).filter(f => f.endsWith('-api-generated.md'));

        for (const file of files) {
            const filePath = path.join(generatedDir, file);
            const content = fs.readFileSync(filePath, 'utf8');

            // Check for empty code blocks or missing code
            const codeStartMatch = content.match(/\[inline-code-start\]/);
            const codeEndMatch = content.match(/\[inline-code-end\]/);

            if (!codeStartMatch || !codeEndMatch) {
                emptyFiles.push({ path: filePath, reason: 'missing code block markers' });
                continue;
            }

            // Extract content between inline-code-start and inline-code-end
            const codeMatch = content.match(/\[inline-code-start\]([\s\S]*?)\[inline-code-end\]/);

            if (codeMatch) {
                const codeContent = codeMatch[1].trim();
                // Check if code is empty or just whitespace
                if (!codeContent || codeContent.length < 10) {
                    emptyFiles.push({ path: filePath, reason: 'empty or minimal code block', codeLength: codeContent.length });
                }
            }
        }
    }

    return emptyFiles;
}

function main() {
    const args = process.argv.slice(2);
    const shouldDelete = args.includes('--delete');

    console.log(shouldDelete ? 'Deleting files...' : 'DRY RUN - files that would be deleted:');
    console.log('');

    const emptyFiles = findEmptyCodeFiles();

    if (emptyFiles.length === 0) {
        console.log('No files with empty code blocks found.');
        return;
    }

    for (const file of emptyFiles) {
        console.log(file.path);
        console.log('  Reason: ' + file.reason);

        if (shouldDelete) {
            fs.unlinkSync(file.path);
            console.log('  DELETED');
        }
        console.log('');
    }

    console.log('Total: ' + emptyFiles.length + ' files ' + (shouldDelete ? 'deleted' : 'would be deleted'));

    if (!shouldDelete) {
        console.log('\nRun with --delete to actually remove the files.');
    }
}

main();
