// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

/**
 * Request to clear output visible on the frontend.
 */
export type ClearOutput = {
  /**
   * The wait flag, which if true, indicates that the frontend should wait
   * for the clear output request to complete before sending further
   * messages.
   */
  wait: boolean;
};