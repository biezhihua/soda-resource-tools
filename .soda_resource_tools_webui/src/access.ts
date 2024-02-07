/**
 * @see https://umijs.org/zh-CN/plugins/plugin-access
 * */
export default function access(initialState: { currentUser?: API.CurrentGetResult } | undefined) {
    const {currentUser} = initialState ?? {};
    return {
        canAdmin: currentUser && currentUser.permission === 'admin',
    };
}
