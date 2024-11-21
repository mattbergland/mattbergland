document.addEventListener('DOMContentLoaded', function() {
    // Guide card interaction
    const guideCards = document.querySelectorAll('.guide-card');
    
    guideCards.forEach((card, index) => {
        card.addEventListener('click', function() {
            const wasActive = this.classList.contains('active');
            const content = document.querySelectorAll('.guide-content')[index];
            
            // First close all cards and content
            guideCards.forEach(c => c.classList.remove('active'));
            document.querySelectorAll('.guide-content').forEach(c => c.classList.remove('active'));
            
            // Then open this card if it wasn't active
            if (!wasActive) {
                this.classList.add('active');
                if (content) {
                    content.classList.add('active');
                }
            }
        });
    });

    // ConvertKit form success message handling
    window.addEventListener('message', e => {
        if (e.data.type === 'convertkit:form:success') {
            const form = document.querySelector('.formkit-form');
            if (form) {
                const successMessage = form.getAttribute('data-success');
                if (successMessage) {
                    const messageDiv = document.createElement('div');
                    messageDiv.className = 'formkit-alert formkit-alert-success';
                    messageDiv.innerHTML = successMessage;
                    form.innerHTML = '';
                    form.appendChild(messageDiv);
                }
            }
        }
    });

    // Smooth scrolling for navigation links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth'
                });
            }
        });
    });
});
