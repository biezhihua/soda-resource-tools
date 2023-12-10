export const setAuthority = (result: API.LoginResult) => {
  let accessToken = JSON.stringify(result);
  window.localStorage.setItem("soda_access_token", accessToken);
}

export const getAuthority = () => {
  let result = window.localStorage.getItem("soda_access_token") || '';
  if (result.length === 0) {
    return null
  } else {
    return JSON.parse(result) as API.LoginResult;
  }
}

export const getAuthorityAccessToken = () => {
  return getAuthority()?.access_token || '';
}
