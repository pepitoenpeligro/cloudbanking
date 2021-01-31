"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.signout = exports.isAuth = exports.authenticate = exports.removeLocalStorage = exports.setLocalStorage = exports.getCookie = exports.removeCookie = exports.setCookie = void 0;

var _jsCookie = _interopRequireDefault(require("js-cookie"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { "default": obj }; }

// set in cookie
var setCookie = function setCookie(key, value) {
  if (window !== 'undefined') {
    _jsCookie["default"].set(key, value, {
      expires: 1
    });
  }
}; // remove from cookie


exports.setCookie = setCookie;

var removeCookie = function removeCookie(key) {
  if (window !== 'undefined') {
    _jsCookie["default"].remove(key, {
      expires: 1
    });
  }
}; // get from cookie such as stored token
// will be useful when we need to make request to server with token


exports.removeCookie = removeCookie;

var getCookie = function getCookie(key) {
  if (window !== 'undefined') {
    return _jsCookie["default"].get(key);
  }
}; // set in localstorage


exports.getCookie = getCookie;

var setLocalStorage = function setLocalStorage(key, value) {
  if (window !== 'undefined') {
    localStorage.setItem(key, JSON.stringify(value));
  }
}; // remove from localstorage


exports.setLocalStorage = setLocalStorage;

var removeLocalStorage = function removeLocalStorage(key) {
  if (window !== 'undefined') {
    localStorage.removeItem(key);
  }
}; // authenticate user by passing data to cookie and localstorage during signin


exports.removeLocalStorage = removeLocalStorage;

var authenticate = function authenticate(response, next) {
  console.log('auth helper signin', response);
  setCookie('token', response.data.token);
  setLocalStorage('user', response.data.user);
  next();
}; // access user info from localstorage


exports.authenticate = authenticate;

var isAuth = function isAuth() {
  if (window !== 'undefined') {
    var cookieChecked = getCookie('token');

    if (cookieChecked) {
      if (localStorage.getItem('user')) {
        return JSON.parse(localStorage.getItem('user'));
      } else {
        return false;
      }
    }
  }
};

exports.isAuth = isAuth;

var signout = function signout(next) {
  removeCookie('token');
  removeLocalStorage('user');
  next();
};

exports.signout = signout;