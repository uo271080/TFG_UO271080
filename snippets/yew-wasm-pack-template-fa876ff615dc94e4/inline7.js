
export function scrollToElement(id) {
  const element = document.getElementById(id);
  if (element) {
    const elementRect = element.getBoundingClientRect();
    const viewportHeight = window.innerHeight || document.documentElement.clientHeight;

    // Calculate scroll position for smooth scrolling to element's midpoint
    const scrollY = elementRect.top + window.pageYOffset - (viewportHeight / 2);

    element.scrollIntoView({
      behavior: 'smooth',
      block: 'center', // Ensure vertical centering
    });
  }
}
