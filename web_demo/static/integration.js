
// Enhanced PDF.js integration with iframe management

class PDFViewerManager {
    constructor() {
        this.iframe = null;
        this.currentPdf = null;
        this.isInitialized = false;
        this.pendingOperations = [];
    }

    async initialize() {
        if (this.isInitialized) return;
        
        // Configure PDF.js
        pdfjsLib.GlobalWorkerOptions.workerSrc = './pdfjs/build/pdf.worker.js';
        
        this.isInitialized = true;
        console.log('PDF Viewer Manager initialized');
        
        // Process any pending operations
        while (this.pendingOperations.length > 0) {
            const operation = this.pendingOperations.shift();
            await operation();
        }
    }

    createViewer(containerId, width, height) {
        if (!this.isInitialized) {
            this.pendingOperations.push(() => this.createViewer(containerId, width, height));
            return false;
        }

        // Create iframe if it doesn't exist
        if (!this.iframe) {
            this.iframe = document.createElement('iframe');
            this.iframe.id = 'pdf-viewer-iframe';
            this.iframe.style.cssText = `
                position: absolute;
                border: none;
                z-index: 1000;
                background: white;
                box-shadow: 0 2px 10px rgba(0,0,0,0.1);
                border-radius: 4px;
                display: none;
            `;
            
            document.body.appendChild(this.iframe);
            
            // Configure PDF.js viewer to open links in new tabs
            this.iframe.onload = () => {
                try {
                    const iframeDoc = this.iframe.contentDocument || this.iframe.contentWindow.document;
                    const script = iframeDoc.createElement('script');
                    script.textContent = `
                        if (window.PDFViewerApplication) {
                            window.PDFViewerApplication.preferences.set('externalLinkTarget', 2);
                        }
                    `;
                    iframeDoc.head.appendChild(script);
                } catch (e) {
                    console.warn('Could not configure PDF.js viewer:', e);
                }
            };
        }

        return true;
    }

    async loadPdfFromBlob(blobUrl) {
        if (!this.isInitialized) {
            return new Promise((resolve, reject) => {
                this.pendingOperations.push(async () => {
                    try {
                        const result = await this.loadPdfFromBlob(blobUrl);
                        resolve(result);
                    } catch (error) {
                        reject(error);
                    }
                });
            });
        }

        if (!this.iframe) {
            throw new Error('PDF viewer not created');
        }

        try {
            // Load PDF in iframe using PDF.js viewer
            const viewerUrl = `./pdfjs/web/viewer.html?file=${encodeURIComponent(blobUrl)}`;
            this.iframe.src = viewerUrl;
            
            // Wait for iframe to load
            await new Promise((resolve, reject) => {
                const timeout = setTimeout(() => reject(new Error('Timeout loading PDF')), 10000);
                
                this.iframe.onload = () => {
                    clearTimeout(timeout);
                    resolve();
                };
                
                this.iframe.onerror = () => {
                    clearTimeout(timeout);
                    reject(new Error('Error loading PDF viewer'));
                };
            });

            console.log('PDF loaded successfully');
            return true;
        } catch (error) {
            console.error('Error loading PDF:', error);
            throw error;
        }
    }

    setViewerPosition(x, y, width, height) {
        if (this.iframe) {
            this.iframe.style.left = x + 'px';
            this.iframe.style.top = y + 'px';
            this.iframe.style.width = width + 'px';
            this.iframe.style.height = height + 'px';
        }
    }

    showViewer(visible) {
        if (this.iframe) {
            this.iframe.style.display = visible ? 'block' : 'none';
        }
    }

    // Page navigation methods
    getCurrentPage() {
        try {
            if (this.iframe && this.iframe.contentWindow && this.iframe.contentWindow.PDFViewerApplication) {
                return this.iframe.contentWindow.PDFViewerApplication.pdfViewer.currentPageNumber || 1;
            }
        } catch (e) {
            console.warn('Could not get current page:', e);
        }
        return 1;
    }

    getTotalPages() {
        try {
            if (this.iframe && this.iframe.contentWindow && this.iframe.contentWindow.PDFViewerApplication) {
                return this.iframe.contentWindow.PDFViewerApplication.pdfViewer.pagesCount || 1;
            }
        } catch (e) {
            console.warn('Could not get total pages:', e);
        }
        return 1;
    }

    setPage(pageNum) {
        try {
            if (this.iframe && this.iframe.contentWindow && this.iframe.contentWindow.PDFViewerApplication) {
                this.iframe.contentWindow.PDFViewerApplication.pdfViewer.currentPageNumber = pageNum;
            }
        } catch (e) {
            console.warn('Could not set page:', e);
        }
    }

    setZoom(zoomLevel) {
        try {
            if (this.iframe && this.iframe.contentWindow && this.iframe.contentWindow.PDFViewerApplication) {
                this.iframe.contentWindow.PDFViewerApplication.pdfViewer.currentScale = zoomLevel;
            }
        } catch (e) {
            console.warn('Could not set zoom:', e);
        }
    }
}

// Global PDF viewer manager
const pdfManager = new PDFViewerManager();

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    pdfManager.initialize();
});

// File picker functionality
window.triggerFilePicker = function(callback) {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.pdf,application/pdf';
    
    input.onchange = function(e) {
        const file = e.target.files[0];
        if (file && file.type === 'application/pdf') {
            callback(file);
        } else {
            console.error('Please select a valid PDF file');
        }
    };
    
    input.click();
};

// Expose functions to WASM
window.createPdfViewer = (containerId, width, height) => pdfManager.createViewer(containerId, width, height);
window.loadPdfFromBlob = (blobUrl) => pdfManager.loadPdfFromBlob(blobUrl);
window.setPdfViewerPosition = (x, y, width, height) => pdfManager.setViewerPosition(x, y, width, height);
window.showPdfViewer = (visible) => pdfManager.showViewer(visible);
window.getCurrentPdfPage = () => pdfManager.getCurrentPage();
window.getTotalPdfPages = () => pdfManager.getTotalPages();
window.setPdfPage = (pageNum) => pdfManager.setPage(pageNum);
window.setPdfZoom = (zoomLevel) => pdfManager.setZoom(zoomLevel);
