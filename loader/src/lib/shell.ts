import { invoke, shell } from '@tauri-apps/api'

export const Shell = {
    /**
     * Expand a folder in file explorer.
     * @param path Absolute path to folder.
     */
    async expandFolder(path: string) {
        await invoke('plugin:shell|expand_folder', {
            path: path
        })
    },

    /**
     * Reveal a file in file explorer.
     * @param path Absolute path to file.
     */
    async revealFile(path: string) {
        await invoke('plugin:shell|reveal_file', {
            path: path
        })
    },

    /**
     * Open an external link.
     * @param url URL.
     */
    async openLink(url: string) {
        if (typeof url === 'string' && url.startsWith('https://')) {
            await shell.open(url)
        }
    },

    /**
     * Extract a zip file to destination directory.
     * @param zipPath Absolute path to zip file.
     * @param destDir Absolute path to destination directory.
     */
    async extractZip(zipPath: string, destDir: string) {
        await invoke('plugin:shell|extract_zip', {
            zipPath,
            destDir
        })
    },

    /**
     * Copy a file to destination path.
     * @param src Absolute path to source file.
     * @param dest Absolute path to destination file.
     */
    async copyFile(src: string, dest: string) {
        await invoke('plugin:shell|copy_file', {
            src,
            dest
        })
    }
}