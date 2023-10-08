#[cfg(target_os = "macos")]
pub const CRATE_DRAG_REGION: &str = r#"
window.addEventListener('load', (event) => {
    let newDiv = document.createElement('div');
    newDiv.setAttribute('data-tauri-drag-region', '');
    newDiv.style.height = '20px';
    newDiv.style.width = 'calc(100% - 70px)';
    newDiv.style.position = 'absolute';
    newDiv.style.top = '0';
    newDiv.style.marginLeft = '70px'
    newDiv.style.zIndex = '999';
    newDiv.style.cursor = 'move';
    newDiv.style.display = 'flex';
    document.body.prepend(newDiv);
});
"#;

#[cfg(not(target_os = "macos"))]
pub const CRATE_DRAG_REGION: &str = r#"
window.addEventListener('load', (event) => {
    let control = document.createElement('div');
    control.className = 'titlebar-container';
    control.style.display = 'flex';
    control.style.justifyContent = 'flex-end';
    document.body.prepend(control);
    control.innerHTML = `
        <div class="titlebar-button" id="titlebar-minimize">
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M20 14H4v-4h16"/></svg>
        </div>
        <div class="titlebar-button" id="titlebar-maximize">
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M4 4h16v16H4V4m2 4v10h12V8H6Z"/></svg>
        </div>
        <div class="titlebar-button" id="titlebar-close">
            <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="currentColor" d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12L19 6.41Z"/></svg>
        </div>
    `;

    let newDiv = document.createElement('div');
    newDiv.setAttribute('data-tauri-drag-region', '');
    newDiv.style.height = '20px';
    newDiv.style.width = 'calc(100% - 70px)';
    newDiv.style.position = 'absolute';
    newDiv.style.top = '0';
    newDiv.style.marginRight = '70px'
    newDiv.style.zIndex = '999';
    newDiv.style.cursor = 'move';

    document.body.prepend(newDiv);

    var appWindow = window.__TAURI__.window.appWindow;
    document
        .getElementById('titlebar-minimize')
        .addEventListener('click', () => appWindow.minimize())
    document
        .getElementById('titlebar-maximize')
        .addEventListener('click', () => appWindow.toggleMaximize())
    document
        .getElementById('titlebar-close')
        .addEventListener('click', () => appWindow.close())

    var css = `
    .titlebar-container {
        position: absolute;
        right: 0;
        z-index: 1000;
    }
    .titlebar {
        height: 30px;
        background: #329ea3;
        user-select: none;
        display: flex;
        justify-content: flex-end;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
    }
    .titlebar-button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 40px;
        height: 30px;
    }
    .titlebar-button:hover {
        background: #64748b;
    }

    @media (prefers-color-scheme: dark) {
        .titlebar-button {
            color: white;
        }
      }
    `;
    var head = document.head || document.getElementsByTagName('head')[0];
    var style = document.createElement('style');

    head.appendChild(style);

    style.type = 'text/css';
    if (style.styleSheet){
        style.styleSheet.cssText = css;
    } else {
        style.appendChild(document.createTextNode(css));
    }
});
"#;

#[cfg(target_os = "macos")]
pub const MODIFY_NAVBAR: &str = r#"
window.addEventListener('load', (event) => {
    function waitForElement(selector, callback) {
      const element = document.querySelector(selector);

      if(element) {
        callback(element);
        return;
      }

      setTimeout(() => waitForElement(selector, callback), 100);
    }

    waitForElement('header', header => {
      header.style.paddingBottom = '10px';
      let firstChild = header.firstElementChild;
      firstChild.style.alignItems = 'end';
    });
});
"#;

#[cfg(not(target_os = "macos"))]
pub const MODIFY_NAVBAR: &str = r#"
window.addEventListener('load', (event) => {
    function waitForElement(selector, callback) {
      const element = document.querySelector(selector);

      if(element) {
        callback(element);
        return;
      }

      setTimeout(() => waitForElement(selector, callback), 100);
    }

    waitForElement('header', header => {
      let navBar = header.firstElementChild;
      navBar.style.alignItems = 'end';
      navBar.firstElementChild.style.alignSelf = 'center';
      navBar.lastElementChild.style.marginBottom = '5px';
    });
});
"#;