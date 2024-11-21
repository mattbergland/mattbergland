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

    // Newsletter subscription form
    const subscribeForm = document.querySelector('.subscribe-form');
    if (subscribeForm) {
        subscribeForm.addEventListener('submit', function(e) {
            e.preventDefault();
            const email = this.querySelector('input[type="email"]').value;
            alert('Thanks for subscribing! We\'ll be in touch soon.');
            this.reset();
        });
    }

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
