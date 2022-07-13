Module.postRun = (() => {
  if (!Module.postRun) {
    return [];
  }

  if (typeof Module.postRun === "function") {
    return [Module.postRun];
  }

  return Module.postRun;
})();

Module.postRun.push((module) => {
  const { throwError } = module;

  module.throwError = () => {
    try {
      return throwError();
    } catch (error) {
      throw typeof error === "number"
        ? new Error(module.getExceptionMessage(error))
        : error;
    }
  };
});
