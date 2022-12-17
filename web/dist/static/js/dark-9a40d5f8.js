import{j as te,aw as ne,ax as oe,ay as ae,ah as L,z as h,az as ue,ag as se,v as G,a0 as p,x as v,av as re,y as V,d as A,ai as x,aA as le,aB as N,aC as ie,W as fe,E as de,aD as ce,K as me,aE as D,aF as he,a9 as F,aG as pe,G as z,o as W,c as Q,f as M}from"./index-2585ad14.js";function ge(t){return function(e,o,n){var a=Object(e);if(!te(e)){var r=ne(o);e=oe(e),o=function(f){return r(a[f],f,a)}}var i=t(e,o,n);return i>-1?a[r?e[i]:i]:void 0}}var ve=ge(ae);const be=ve;function Te(){const{$storage:t,$config:e}=L(),o=()=>{var r,i,f,m,c,g,l,s,d,b,B;ue().multiTagsCache&&(!t.tags||t.tags.length===0)&&(t.tags=se),t.layout||(t.layout={layout:(r=e==null?void 0:e.Layout)!=null?r:"vertical",theme:(i=e==null?void 0:e.Theme)!=null?i:"default",darkMode:(f=e==null?void 0:e.DarkMode)!=null?f:!1,sidebarStatus:(m=e==null?void 0:e.SidebarStatus)!=null?m:!0,epThemeColor:(c=e==null?void 0:e.EpThemeColor)!=null?c:"#409EFF"}),t.configure||(t.configure={grey:(g=e==null?void 0:e.Grey)!=null?g:!1,weak:(l=e==null?void 0:e.Weak)!=null?l:!1,hideTabs:(s=e==null?void 0:e.HideTabs)!=null?s:!1,showLogo:(d=e==null?void 0:e.ShowLogo)!=null?d:!0,showModel:(b=e==null?void 0:e.ShowModel)!=null?b:"smart",multiTagsCache:(B=e==null?void 0:e.MultiTagsCache)!=null?B:!1})},n=h(()=>t==null?void 0:t.layout.layout),a=h(()=>t.layout);return{layout:n,layoutTheme:a,initStorage:o}}const ye=G({id:"pure-app",state:()=>{var t,e,o,n;return{sidebar:{opened:(e=(t=p().getItem("responsive-layout"))==null?void 0:t.sidebarStatus)!=null?e:v().SidebarStatus,withoutAnimation:!1,isClickCollapse:!1},layout:(n=(o=p().getItem("responsive-layout"))==null?void 0:o.layout)!=null?n:v().Layout,device:re()?"mobile":"desktop"}},getters:{getSidebarStatus(){return this.sidebar.opened},getDevice(){return this.device}},actions:{TOGGLE_SIDEBAR(t,e){const o=p().getItem("responsive-layout");t&&e?(this.sidebar.withoutAnimation=!0,this.sidebar.opened=!0,o.sidebarStatus=!0):!t&&e?(this.sidebar.withoutAnimation=!0,this.sidebar.opened=!1,o.sidebarStatus=!1):!t&&!e&&(this.sidebar.withoutAnimation=!1,this.sidebar.opened=!this.sidebar.opened,this.sidebar.isClickCollapse=!this.sidebar.opened,o.sidebarStatus=this.sidebar.opened),p().setItem("responsive-layout",o)},async toggleSideBar(t,e){await this.TOGGLE_SIDEBAR(t,e)},toggleDevice(t){this.device=t},setLayout(t){this.layout=t}}});function $e(){return ye(V)}const Ce=G({id:"pure-epTheme",state:()=>{var t,e,o,n;return{epThemeColor:(e=(t=p().getItem("responsive-layout"))==null?void 0:t.epThemeColor)!=null?e:v().EpThemeColor,epTheme:(n=(o=p().getItem("responsive-layout"))==null?void 0:o.theme)!=null?n:v().Theme}},getters:{getEpThemeColor(){return this.epThemeColor},fill(){return this.epTheme==="light"?"#409eff":this.epTheme==="yellow"?"#d25f00":"#fff"}},actions:{setEpThemeColor(t){const e=p().getItem("responsive-layout");this.epTheme=e==null?void 0:e.theme,this.epThemeColor=t,e&&(e.epThemeColor=t,p().setItem("responsive-layout",e))}}});function I(){return Ce(V)}function Oe(t,e){const o=/^IF-/;if(o.test(t)){const n=t.split(o)[1],a=n.slice(0,n.indexOf(" ")==-1?n.length:n.indexOf(" ")),r=n.slice(n.indexOf(" ")+1,n.length);return A({name:"FontIcon",render(){return x(le,{icon:a,iconType:r,...e})}})}else return typeof t=="function"||typeof(t==null?void 0:t.render)=="function"?t:typeof t=="object"?A({name:"OfflineIcon",render(){return x(N,{icon:t,...e})}}):A({name:"Icon",render(){const n=t&&t.includes(":")?ie:N;return x(n,{icon:t,...e})}})}const O="\u5F53\u524D\u8DEF\u7531\u914D\u7F6E\u4E0D\u6B63\u786E\uFF0C\u8BF7\u68C0\u67E5\u914D\u7F6E";function Pe(){var w,H;const t=fe(),e=$e(),o=de().options.routes,{wholeMenus:n}=ce(me()),a=(H=(w=v())==null?void 0:w.TooltipEffect)!=null?H:"light",r=h(()=>{var u;return(u=D())==null?void 0:u.username}),i=h(()=>r.value?{marginRight:"10px"}:""),f=h(()=>!e.getSidebarStatus),m=h(()=>e.getDevice),{$storage:c,$config:g}=L(),l=h(()=>{var u;return(u=c==null?void 0:c.layout)==null?void 0:u.layout}),s=h(()=>g.Title);function d(u){const T=v().Title;T?document.title=`${u.title} | ${T}`:document.title=u.title}function b(){D().logOut()}function B(){he.push("/welcome")}function q(){F.emit("openPanel")}function J(){e.toggleSideBar()}function X(u){u==null||u.handleResize()}function Y(u){var C;if(!u.children)return console.error(O);const T=/^http(s?):\/\//,y=(C=u.children[0])==null?void 0:C.path;return T.test(y)?u.path+"/"+y:y}function Z(u,T){if(n.value.length===0||ee(u))return;let y="";const C=u.lastIndexOf("/");C>0&&(y=u.slice(0,C));function S(k,_){return _?_.map($=>{$.path===k?$.redirect?S($.redirect,$.children):F.emit("changLayoutRoute",{indexPath:k,parentPath:y}):$.children&&S(k,$.children)}):console.error(O)}S(u,T)}function ee(u){return pe.includes(u)}return{route:t,title:s,device:m,layout:l,logout:b,routers:o,$storage:c,backHome:B,onPanel:q,changeTitle:d,toggleSideBar:J,menuSelect:Z,handleResize:X,resolvePath:Y,isCollapse:f,pureApp:e,username:r,avatarsStyle:i,tooltipEffect:a}}const E={outputDir:"",defaultScopeName:"",includeStyleWithColors:[],extract:!0,themeLinkTagId:"head",themeLinkTagInjectTo:"head",removeCssScopeName:!1,customThemeCssFileName:null,arbitraryMode:!1,defaultPrimaryColor:"",customThemeOutputPath:"/workspace/pure-admin-thin/node_modules/.pnpm/@pureadmin+theme@2.4.0/node_modules/@pureadmin/theme/setCustomTheme.js",styleTagId:"custom-theme-tagid",InjectDefaultStyleTagToHtml:!0,hueDiffControls:{low:0,high:0},multipleScopeVars:[{scopeName:"layout-theme-default",varsContent:`
        $subMenuActiveText: #fff !default;
        $menuBg: #001529 !default;
        $menuHover: #4091f7 !default;
        $subMenuBg: #0f0303 !default;
        $subMenuActiveBg: #4091f7 !default;
        $menuText: rgb(254 254 254 / 65%) !default;
        $sidebarLogo: #002140 !default;
        $menuTitleHover: #fff !default;
        $menuActiveBefore: #4091f7 !default;
      `},{scopeName:"layout-theme-light",varsContent:`
        $subMenuActiveText: #409eff !default;
        $menuBg: #fff !default;
        $menuHover: #e0ebf6 !default;
        $subMenuBg: #fff !default;
        $subMenuActiveBg: #e0ebf6 !default;
        $menuText: #7a80b4 !default;
        $sidebarLogo: #fff !default;
        $menuTitleHover: #000 !default;
        $menuActiveBefore: #4091f7 !default;
      `},{scopeName:"layout-theme-dusk",varsContent:`
        $subMenuActiveText: #fff !default;
        $menuBg: #2a0608 !default;
        $menuHover: #e13c39 !default;
        $subMenuBg: #000 !default;
        $subMenuActiveBg: #e13c39 !default;
        $menuText: rgb(254 254 254 / 65.1%) !default;
        $sidebarLogo: #42090c !default;
        $menuTitleHover: #fff !default;
        $menuActiveBefore: #e13c39 !default;
      `},{scopeName:"layout-theme-volcano",varsContent:`
        $subMenuActiveText: #fff !default;
        $menuBg: #2b0e05 !default;
        $menuHover: #e85f33 !default;
        $subMenuBg: #0f0603 !default;
        $subMenuActiveBg: #e85f33 !default;
        $menuText: rgb(254 254 254 / 65%) !default;
        $sidebarLogo: #441708 !default;
        $menuTitleHover: #fff !default;
        $menuActiveBefore: #e85f33 !default;
      `},{scopeName:"layout-theme-yellow",varsContent:`
        $subMenuActiveText: #d25f00 !default;
        $menuBg: #2b2503 !default;
        $menuHover: #f6da4d !default;
        $subMenuBg: #0f0603 !default;
        $subMenuActiveBg: #f6da4d !default;
        $menuText: rgb(254 254 254 / 65%) !default;
        $sidebarLogo: #443b05 !default;
        $menuTitleHover: #fff !default;
        $menuActiveBefore: #f6da4d !default;
      `},{scopeName:"layout-theme-mingQing",varsContent:`
        $subMenuActiveText: #fff !default;
        $menuBg: #032121 !default;
        $menuHover: #59bfc1 !default;
        $subMenuBg: #000 !default;
        $subMenuActiveBg: #59bfc1 !default;
        $menuText: #7a80b4 !default;
        $sidebarLogo: #053434 !default;
        $menuTitleHover: #fff !default;
        $menuActiveBefore: #59bfc1 !default;
      `},{scopeName:"layout-theme-auroraGreen",varsContent:`
        $subMenuActiveText: #fff !default;
        $menuBg: #0b1e15 !default;
        $menuHover: #60ac80 !default;
        $subMenuBg: #000 !default;
        $subMenuActiveBg: #60ac80 !default;
        $menuText: #7a80b4 !default;
        $sidebarLogo: #112f21 !default;
        $menuTitleHover: #fff !default;
        $menuActiveBefore: #60ac80 !default;
      `},{scopeName:"layout-theme-pink",varsContent:`
        $subMenuActiveText: #fff !default;
        $menuBg: #28081a !default;
        $menuHover: #d84493 !default;
        $subMenuBg: #000 !default;
        $subMenuActiveBg: #d84493 !default;
        $menuText: #7a80b4 !default;
        $sidebarLogo: #3f0d29 !default;
        $menuTitleHover: #fff !default;
        $menuActiveBefore: #d84493 !default;
      `},{scopeName:"layout-theme-saucePurple",varsContent:`
        $subMenuActiveText: #fff !default;
        $menuBg: #130824 !default;
        $menuHover: #693ac9 !default;
        $subMenuBg: #000 !default;
        $subMenuActiveBg: #693ac9 !default;
        $menuText: #7a80b4 !default;
        $sidebarLogo: #1f0c38 !default;
        $menuTitleHover: #fff !default;
        $menuActiveBefore: #693ac9 !default;
      `}]},Be="/",Me="assets";function K(t){let e=t.replace("#","").match(/../g);for(let o=0;o<3;o++)e[o]=parseInt(e[o],16);return e}function U(t,e,o){let n=[t.toString(16),e.toString(16),o.toString(16)];for(let a=0;a<3;a++)n[a].length==1&&(n[a]=`0${n[a]}`);return`#${n.join("")}`}function Se(t,e){let o=K(t);for(let n=0;n<3;n++)o[n]=Math.floor(o[n]*(1-e));return U(o[0],o[1],o[2])}function ke(t,e){let o=K(t);for(let n=0;n<3;n++)o[n]=Math.floor((255-o[n])*e+o[n]);return U(o[0],o[1],o[2])}function P(t){return`(^${t}\\s+|\\s+${t}\\s+|\\s+${t}$|^${t}$)`}function R({scopeName:t,multipleScopeVars:e}){const o=Array.isArray(e)&&e.length?e:E.multipleScopeVars;let n=document.documentElement.className;new RegExp(P(t)).test(n)||(o.forEach(a=>{n=n.replace(new RegExp(P(a.scopeName),"g"),` ${t} `)}),document.documentElement.className=n.replace(/(^\s+|\s+$)/g,""))}function j({id:t,href:e}){const o=document.createElement("link");return o.rel="stylesheet",o.href=e,o.id=t,o}function Ae(t){const e={scopeName:"theme-default",customLinkHref:r=>r,...t},o=e.themeLinkTagId||E.themeLinkTagId;let n=document.getElementById(o);const a=e.customLinkHref(`/${Be}/${Me}/${e.scopeName}.css`.replace(/\/+(?=\/)/g,""));if(n){n.id=`${o}_old`;const r=j({id:o,href:a});n.nextSibling?n.parentNode.insertBefore(r,n.nextSibling):n.parentNode.appendChild(r),r.onload=()=>{setTimeout(()=>{n.parentNode.removeChild(n),n=null},60),R(e)};return}n=j({id:o,href:a}),R(e),document[(e.themeLinkTagInjectTo||E.themeLinkTagInjectTo||"").replace("-prepend","")].appendChild(n)}function Re(){var g;const{layoutTheme:t,layout:e}=Te(),o=z([{color:"#1b2a47",themeColor:"default"},{color:"#ffffff",themeColor:"light"},{color:"#f5222d",themeColor:"dusk"},{color:"#fa541c",themeColor:"volcano"},{color:"#fadb14",themeColor:"yellow"},{color:"#13c2c2",themeColor:"mingQing"},{color:"#52c41a",themeColor:"auroraGreen"},{color:"#eb2f96",themeColor:"pink"},{color:"#722ed1",themeColor:"saucePurple"}]),{$storage:n}=L(),a=z((g=n==null?void 0:n.layout)==null?void 0:g.darkMode),r=document.documentElement;function i(l="default"){var s,d;if(t.value.theme=l,Ae({scopeName:`layout-theme-${l}`}),n.layout={layout:e.value,theme:l,darkMode:a.value,sidebarStatus:(s=n.layout)==null?void 0:s.sidebarStatus,epThemeColor:(d=n.layout)==null?void 0:d.epThemeColor},l==="default"||l==="light")m(v().EpThemeColor);else{const b=be(o.value,{themeColor:l});m(b.color)}}function f(l,s,d){document.documentElement.style.setProperty(`--el-color-primary-${l}-${s}`,a.value?Se(d,s/10):ke(d,s/10))}const m=l=>{I().setEpThemeColor(l),document.documentElement.style.setProperty("--el-color-primary",l);for(let s=1;s<=2;s++)f("dark",s,l);for(let s=1;s<=9;s++)f("light",s,l)};function c(){I().epTheme==="light"&&a.value?i("default"):i(I().epTheme),a.value?document.documentElement.classList.add("dark"):document.documentElement.classList.remove("dark")}return{body:r,dataTheme:a,layoutTheme:t,themeColors:o,dataThemeChange:c,setEpThemeColor:m,setLayoutThemeColor:i}}const xe={xmlns:"http://www.w3.org/2000/svg",width:"16",height:"16",viewBox:"0 0 24 24"},Ie=M("path",{fill:"none",d:"M0 0h24v24H0z"},null,-1),Ee=M("path",{d:"M12 18a6 6 0 1 1 0-12 6 6 0 0 1 0 12zM11 1h2v3h-2V1zm0 19h2v3h-2v-3zM3.515 4.929l1.414-1.414L7.05 5.636 5.636 7.05 3.515 4.93zM16.95 18.364l1.414-1.414 2.121 2.121-1.414 1.414-2.121-2.121zm2.121-14.85 1.414 1.415-2.121 2.121-1.414-1.414 2.121-2.121zM5.636 16.95l1.414 1.414-2.121 2.121-1.414-1.414 2.121-2.121zM23 11v2h-3v-2h3zM4 11v2H1v-2h3z"},null,-1),Le=[Ie,Ee];function we(t,e){return W(),Q("svg",xe,Le)}const je={render:we},He={xmlns:"http://www.w3.org/2000/svg",width:"16",height:"16",viewBox:"0 0 24 24"},_e=M("path",{fill:"none",d:"M0 0h24v24H0z"},null,-1),Ne=M("path",{d:"M11.38 2.019a7.5 7.5 0 1 0 10.6 10.6C21.662 17.854 17.316 22 12.001 22 6.477 22 2 17.523 2 12c0-5.315 4.146-9.661 9.38-9.981z"},null,-1),De=[_e,Ne];function Fe(t,e){return W(),Q("svg",He,De)}const Ge={render:Fe};export{Oe as a,Pe as b,Re as c,$e as d,je as e,Ge as f,Te as g,Ae as t,I as u};
