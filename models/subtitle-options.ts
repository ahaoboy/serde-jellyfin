/* tslint:disable */
/* eslint-disable */
/**
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * Do not edit the class manually.
 *
 * Jellyfin API
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */



/**
 * 
 * @export
 * @interface SubtitleOptions
 */
export interface SubtitleOptions {
    /**
     * 
     * @type {boolean}
     * @memberof SubtitleOptions
     */
    'SkipIfEmbeddedSubtitlesPresent'?: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof SubtitleOptions
     */
    'SkipIfAudioTrackMatches'?: boolean;
    /**
     * 
     * @type {Array<string>}
     * @memberof SubtitleOptions
     */
    'DownloadLanguages'?: Array<string> | null;
    /**
     * 
     * @type {boolean}
     * @memberof SubtitleOptions
     */
    'DownloadMovieSubtitles'?: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof SubtitleOptions
     */
    'DownloadEpisodeSubtitles'?: boolean;
    /**
     * 
     * @type {string}
     * @memberof SubtitleOptions
     */
    'OpenSubtitlesUsername'?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SubtitleOptions
     */
    'OpenSubtitlesPasswordHash'?: string | null;
    /**
     * 
     * @type {boolean}
     * @memberof SubtitleOptions
     */
    'IsOpenSubtitleVipAccount'?: boolean;
    /**
     * 
     * @type {boolean}
     * @memberof SubtitleOptions
     */
    'RequirePerfectMatch'?: boolean;
}

