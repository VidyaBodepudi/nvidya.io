// Matrix Rain Effect for Sidebar
const canvas = document.getElementById('matrix-canvas');
const ctx = canvas.getContext('2d');

function resizeCanvas() {
    canvas.width = canvas.parentElement.offsetWidth;
    canvas.height = canvas.parentElement.offsetHeight;
}
resizeCanvas();
window.addEventListener('resize', resizeCanvas);

const katakana = 'ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ';
const nums = '0123456789';
const symbols = ':・."=*+-<>¦|';
const allChars = katakana + nums + symbols;
const fontSize = 16;

// Weighted color pool — classic matrix green is heavily favored (~50%)
// to keep the theme coherent while other colors add variety
const charColors = [
    '#00ff41', '#00ff41', '#00ff41', '#00ff41', '#00ff41',  // Classic Matrix Green (5x weight)
    '#00ffff',  // Cyan
    '#ffff00',  // Electric Yellow
    '#ff1493',  // Neon Pink
    '#ff6600',  // Neon Orange
    '#bf00ff',  // Electric Violet
    '#ff0044',  // Hot Red
    '#0088ff',  // Electric Blue
];

// Brighter lead versions of each color
const leadVersions = {
    '#00ff41': '#aaffaa',
    '#00ffff': '#ccffff',
    '#ffff00': '#ffffaa',
    '#ff1493': '#ffaaff',
    '#ff6600': '#ffd699',
    '#bf00ff': '#ddaaff',
    '#ff0044': '#ffaaaa',
    '#0088ff': '#aaddff',
};

function randomColor() {
    return charColors[Math.floor(Math.random() * charColors.length)];
}

let columns, drops;

function initDrops() {
    columns = Math.floor(canvas.width / fontSize);
    drops = [];
    for (let x = 0; x < columns; x++) {
        // Start drops at random positions across the full height so rain is
        // immediately visible instead of waiting for drops to scroll in
        drops[x] = Math.floor(Math.random() * (canvas.height / fontSize));
    }
}
initDrops();
window.addEventListener('resize', () => { resizeCanvas(); initDrops(); });

function drawMatrix() {
    // Semi-transparent black overlay to create the fading trail effect
    // Lower alpha = longer trails. 0.04 gives nice long trails.
    ctx.fillStyle = 'rgba(3, 5, 8, 0.04)';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.font = fontSize + 'px monospace';

    for (let i = 0; i < columns; i++) {
        const char = allChars.charAt(Math.floor(Math.random() * allChars.length));
        const x = i * fontSize;
        const y = drops[i] * fontSize;

        // Lead character — pick a random color and use its bright version
        const color = randomColor();
        ctx.fillStyle = leadVersions[color];
        ctx.fillText(char, x, y);

        // Trail character one step behind — independently colored
        if (drops[i] > 1) {
            const trailChar = allChars.charAt(Math.floor(Math.random() * allChars.length));
            ctx.fillStyle = randomColor();
            ctx.fillText(trailChar, x, y - fontSize);
        }

        // Reset drop to top after it passes the bottom, with some randomness
        if (y > canvas.height && Math.random() > 0.95) {
            drops[i] = 0;
        }

        drops[i]++;
    }
}

// Run the animation at ~30fps
setInterval(drawMatrix, 33);

// Typing effect for main hero text
const heroText = "Initializing system...";
const typingElement = document.querySelector('.typing-effect');
let typingTimeout;

function typeText() {
    typingElement.textContent = '';
    let textIndex = 0;

    function type() {
        if (textIndex < heroText.length) {
            typingElement.textContent += heroText.charAt(textIndex);
            textIndex++;
            typingTimeout = setTimeout(type, 100 + Math.random() * 50);
        }
    }
    type();
}

// Start typing effect shortly after load
setTimeout(typeText, 800);

// Smooth scrolling for nav links
document.querySelectorAll('.nav-links a').forEach(anchor => {
    anchor.addEventListener('click', function(e) {
        e.preventDefault();

        // Update active state
        document.querySelectorAll('.nav-links a').forEach(a => a.classList.remove('active'));
        this.classList.add('active');

        const targetId = this.getAttribute('href').substring(1);
        const targetSection = document.getElementById(targetId);

        if (targetSection) {
            window.scrollTo({
                top: targetSection.offsetTop - 40,
                behavior: 'smooth'
            });
        }
    });
});

// Intersection Observer for scroll animations and active nav links
const sections = document.querySelectorAll('.section');
const navLinks = document.querySelectorAll('.nav-links a');

const observerOptions = {
    root: null,
    rootMargin: '0px',
    threshold: 0.2
};

const sectionObserver = new IntersectionObserver((entries, observer) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            // Add visible class for fade-in animation
            entry.target.classList.add('visible');

            // Update active nav link
            const id = entry.target.getAttribute('id');
            navLinks.forEach(link => {
                link.classList.remove('active');
                if (link.getAttribute('href') === `#${id}`) {
                    link.classList.add('active');
                }
            });
        }
    });
}, observerOptions);

sections.forEach(section => {
    sectionObserver.observe(section);
});

// Add glitch effect on hover to the profile name
const profileName = document.querySelector('.glitch');
profileName.addEventListener('mouseover', () => {
    profileName.style.animation = 'none';
    setTimeout(() => {
        profileName.style.animation = '';
    }, 10);
});
